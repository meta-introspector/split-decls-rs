// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/sgx/waitqueue/unsafe_list.rs
// Error: expected square brackets
// Problematic line: line 4

//! A doubly-linked list where callers are in charge of memory allocation
//! of the nodes in the list.

#[cfg(test)]
mod tests;

use crate::mem;
