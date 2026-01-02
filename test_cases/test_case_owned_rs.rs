// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/fd/owned.rs
// Error: expected square brackets
// Problematic line: line 7

#![deny(unsafe_op_in_unsafe_fn)]

use super::raw::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(not(target_os = "trusty"))]
use crate::fs;
use crate::marker::PhantomData;
use crate::mem::ManuallyDrop;
