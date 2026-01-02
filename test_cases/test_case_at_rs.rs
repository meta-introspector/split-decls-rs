// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_infer/src/infer/at.rs
// Error: expected square brackets
// Problematic line: line 39

use crate::traits::Obligation;
use crate::traits::solve::Goal;

/// Whether we should define opaque types or just treat them opaquely.
///
/// Currently only used to prevent predicate matching from matching anything
/// against opaque types.
