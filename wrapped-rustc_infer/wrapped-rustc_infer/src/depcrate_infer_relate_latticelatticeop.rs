// Generated macro for LatticeOp (struct)
macro_rules! Depcrate_infer_relate_latticeLatticeOp {
() => {
// Module: crate::infer::relate::lattice
// Provides: {"LatticeOp"}
// Dependencies: {}
# [doc = " A greatest lower bound\" (common subtype) or least upper bound (common supertype)."] pub (crate) struct LatticeOp < 'infcx , 'tcx > { infcx : & 'infcx InferCtxt < 'tcx > , trace : TypeTrace < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , kind : LatticeOpKind , obligations : PredicateObligations < 'tcx > , }
};
}
