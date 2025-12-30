// Generated macro for impl_261 (impl)
macro_rules! Depcrate_infer_outlives_obligationsimpl_261 {
() => {
// Module: crate::infer::outlives::obligations
// Provides: {"impl_261"}
// Dependencies: {}
impl < 'cx , 'tcx > TypeOutlivesDelegate < 'tcx > for & 'cx InferCtxt < 'tcx > { fn push_sub_region_constraint (& mut self , origin : SubregionOrigin < 'tcx > , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > , _constraint_category : ConstraintCategory < 'tcx > ,) { self . sub_regions (origin , a , b) } fn push_verify (& mut self , origin : SubregionOrigin < 'tcx > , kind : GenericKind < 'tcx > , a : ty :: Region < 'tcx > , bound : VerifyBound < 'tcx > ,) { self . verify_generic_bound (origin , kind , a , bound) } }
};
}
