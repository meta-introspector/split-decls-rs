// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/io/stdio.rs
// Error: expected square brackets
// Problematic line: line 3

#![cfg_attr(test, allow(unused))]

#[cfg(test)]
mod tests;

use crate::cell::{Cell, RefCell};
