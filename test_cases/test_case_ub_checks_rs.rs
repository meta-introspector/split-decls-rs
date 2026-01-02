// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ub_checks.rs
// Error: expected square brackets
// Problematic line: line 6


use crate::intrinsics::{self, const_eval_select};

/// Checks that the preconditions of an unsafe function are followed.
///
/// The check is enabled at runtime if debug assertions are enabled when the
/// caller is monomorphized. In const-eval/Miri checks implemented with this
