// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/diagnostics/outlives_suggestion.rs
// Error: expected square brackets
// Problematic line: line 18

use super::{ErrorConstraintInfo, RegionName, RegionNameSource};
use crate::MirBorrowckCtxt;

/// The different things we could suggest.
enum SuggestedConstraint {
    /// Outlives(a, [b, c, d, ...]) => 'a: 'b + 'c + 'd + ...
    Outlives(RegionName, SmallVec<[RegionName; 2]>),
