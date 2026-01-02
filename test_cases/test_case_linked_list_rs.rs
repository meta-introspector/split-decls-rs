// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/collections/linked_list.rs
// Error: expected square brackets
// Problematic line: line 26

use crate::alloc::{Allocator, Global};
use crate::boxed::Box;

#[cfg(test)]
mod tests;

/// A doubly-linked list with owned nodes.
