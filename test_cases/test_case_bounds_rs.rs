// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/intrinsics/bounds.rs
// Error: expected square brackets
// Problematic line: line 5


use crate::marker::PointeeSized;

/// Types with a built-in dereference operator in runtime MIR,
/// aka references and raw pointers.
///
/// # Safety
