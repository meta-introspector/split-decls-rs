// Generated macro for impl_407 (impl)
macro_rules! Depcrate_infer_relate_latticeimpl_407 {
() => {
// Module: crate::infer::relate::lattice
// Provides: {"impl_407"}
// Dependencies: {}
impl < 'infcx , 'tcx > LatticeOp < 'infcx , 'tcx > { pub (crate) fn new (infcx : & 'infcx InferCtxt < 'tcx > , trace : TypeTrace < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , kind : LatticeOpKind ,) -> LatticeOp < 'infcx , 'tcx > { LatticeOp { infcx , trace , param_env , kind , obligations : PredicateObligations :: new () } } pub (crate) fn into_obligations (self) -> PredicateObligations < 'tcx > { self . obligations } }
};
}
