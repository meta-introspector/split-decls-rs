// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/different_lifetimes.rs
// Error: expected square brackets
// Problematic line: line 16

use crate::errors::{AddLifetimeParamsSuggestion, LifetimeMismatch, LifetimeMismatchLabels};
use crate::infer::{RegionResolutionError, SubregionOrigin};

impl<'a, 'tcx> NiceRegionError<'a, 'tcx> {
    /// Print the error message for lifetime errors when both the concerned regions are anonymous.
    ///
    /// Consider a case where we have
