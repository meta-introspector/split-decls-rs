// Generated macro for TypeOutlivesDelegate (trait)
macro_rules! Depcrate_infer_outlives_obligationsTypeOutlivesDelegate {
() => {
// Module: crate::infer::outlives::obligations
// Provides: {"TypeOutlivesDelegate"}
// Dependencies: {}
pub trait TypeOutlivesDelegate < 'tcx > { fn push_sub_region_constraint (& mut self , origin : SubregionOrigin < 'tcx > , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > , constraint_category : ConstraintCategory < 'tcx > ,) ; fn push_verify (& mut self , origin : SubregionOrigin < 'tcx > , kind : GenericKind < 'tcx > , a : ty :: Region < 'tcx > , bound : VerifyBound < 'tcx > ,) ; }
};
}
