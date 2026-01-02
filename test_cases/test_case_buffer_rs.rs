// MINIMAL TEST CASE for parsing failure in: ../rust/library/proc_macro/src/bridge/buffer.rs
// Error: expected square brackets
// Problematic line: line 8

use std::ops::{Deref, DerefMut};
use std::slice;

#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize,
