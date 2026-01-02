// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ops/deref.rs
// Error: expected identifier or `_`
// Problematic line: line 3

use crate::marker::PointeeSized;

/// Used for immutable dereferencing operations, like `*v`.
///
/// In addition to being used for explicit dereferencing operations with the
/// (unary) `*` operator in immutable contexts, `Deref` is also used implicitly
