// SRC: ../rust/library/std/src/sys/pal/unsupported/mod.rs
#![deny(unsafe_op_in_unsafe_fn)]

pub mod os;
pub mod pipe;
pub mod thread;
pub mod time;

mod common;
pub use common::*;
