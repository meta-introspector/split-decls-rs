// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_pattern_analysis/src/checks.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::pat_column::PatternColumn;
use crate::{MatchArm, PatCx};

/// Validate that deref patterns and normal constructors aren't used to match on the same place.
pub(crate) fn detect_mixed_deref_pat_ctors<'p, Cx: PatCx>(
    cx: &Cx,
    arms: &[MatchArm<'p, Cx>],
