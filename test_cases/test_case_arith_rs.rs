// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ops/arith.rs
// Error: expected identifier or `_`
// Problematic line: line 1

/// The addition operator `+`.
///
/// Note that `Rhs` is `Self` by default, but this is not mandatory. For
/// example, [`std::time::SystemTime`] implements `Add<Duration>`, which permits
