// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ops/try_trait.rs
// Error: expected identifier or `_`
// Problematic line: line 3

use crate::ops::ControlFlow;

/// The `?` operator and `try {}` blocks.
///
/// `try_*` methods typically involve a type implementing this trait.  For
/// example, the closures passed to [`Iterator::try_fold`] and
