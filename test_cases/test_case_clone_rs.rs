// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/clone.rs
// Error: expected square brackets
// Problematic line: line 43


mod uninit;

/// A common trait that allows explicit creation of a duplicate value.
///
/// Calling [`clone`] always produces a new value.
/// However, for types that are references to other data (such as smart pointers or references),
