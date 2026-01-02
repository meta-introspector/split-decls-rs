// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ops/index.rs
// Error: expected identifier or `_`
// Problematic line: line 1

/// Used for indexing operations (`container[index]`) in immutable contexts.
///
/// `container[index]` is actually syntactic sugar for `*container.index(index)`,
/// but only when used as an immutable value. If a mutable value is requested,
