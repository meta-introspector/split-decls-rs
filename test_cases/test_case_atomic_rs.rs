// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/sync/atomic.rs
// Error: expected square brackets
// Problematic line: line 253


trait Sealed {}

/// A marker trait for primitive types which can be modified atomically.
///
/// This is an implementation detail for <code>[Atomic]\<T></code> which may disappear or be replaced at any time.
///
