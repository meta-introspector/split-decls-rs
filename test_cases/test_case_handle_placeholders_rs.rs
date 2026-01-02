// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/handle_placeholders.rs
// Error: expected square brackets
// Problematic line: line 25

use crate::universal_regions::UniversalRegions;
use crate::{BorrowckInferCtxt, NllRegionVariableOrigin};

/// A set of outlives constraints after rewriting to remove
/// higher-kinded constraints.
pub(crate) struct LoweredConstraints<'tcx> {
    pub(crate) constraint_sccs: Sccs<RegionVid, ConstraintSccIndex>,
