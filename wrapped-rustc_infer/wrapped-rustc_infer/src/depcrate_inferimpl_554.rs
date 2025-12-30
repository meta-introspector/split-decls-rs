// Generated macro for impl_554 (impl)
macro_rules! Depcrate_inferimpl_554 {
() => {
// Module: crate::infer
// Provides: {"impl_554"}
// Dependencies: {}
impl < 'tcx > SubregionOrigin < 'tcx > { pub fn to_constraint_category (& self) -> ConstraintCategory < 'tcx > { match self { Self :: Subtype (type_trace) => type_trace . cause . to_constraint_category () , Self :: AscribeUserTypeProvePredicate (span) => ConstraintCategory :: Predicate (* span) , _ => ConstraintCategory :: BoringNoLocation , } } }
};
}
