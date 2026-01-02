// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/named_anon_conflict.rs
// Error: expected square brackets
// Problematic line: line 11

use crate::error_reporting::infer::nice_region_error::find_anon_type::find_anon_type;
use crate::errors::ExplicitLifetimeRequired;

impl<'a, 'tcx> NiceRegionError<'a, 'tcx> {
    /// When given a `ConcreteFailure` for a function with parameters containing a named region and
    /// an anonymous region, emit an descriptive diagnostic error.
    pub(super) fn try_report_named_anon_conflict(&self) -> Option<Diag<'tcx>> {
