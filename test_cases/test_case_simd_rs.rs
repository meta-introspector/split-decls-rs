// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/intrinsics/simd.rs
// Error: expected square brackets
// Problematic line: line 5

//!
//! In this module, a "vector" is any `repr(simd)` type.

/// Inserts an element into a vector, returning the updated vector.
///
/// `T` must be a vector with element type `U`, and `idx` must be `const`.
///
