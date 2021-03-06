//! Copyright (c) 2016-2017, The Tor Project, Inc. */
//! See LICENSE for licensing information */

//! Interface for external calls to tor C ABI
//!
//! The purpose of this module is to provide a clean interface for when Rust
//! modules need to interact with functionality in tor C code rather than each
//! module implementing this functionality repeatedly.

extern crate libc;

mod external;

pub use external::*;
