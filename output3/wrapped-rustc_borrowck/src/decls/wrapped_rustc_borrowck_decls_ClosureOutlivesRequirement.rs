use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Indicates an outlives-constraint between a type or between two
/// free regions declared on the closure.
#[derive(Copy, Clone, Debug)]
pub struct ClosureOutlivesRequirement<'tcx> {
    pub subject: ClosureOutlivesSubject<'tcx>,
    pub outlived_free_region: ty::RegionVid,
    pub blame_span: Span,
    pub category: ConstraintCategory<'tcx>,
}
