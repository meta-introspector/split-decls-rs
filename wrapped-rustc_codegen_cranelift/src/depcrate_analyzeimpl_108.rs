// Generated macro for impl_108 (impl)
macro_rules! Depcrate_analyzeimpl_108 {
() => {
// Module: crate::analyze
// Provides: {"impl_108"}
// Dependencies: {}
impl SsaKind { pub (crate) fn is_ssa < 'tcx > (self , fx : & FunctionCx < '_ , '_ , 'tcx > , ty : Ty < 'tcx >) -> bool { self == SsaKind :: MaybeSsa && (fx . clif_type (ty) . is_some () || fx . clif_pair_type (ty) . is_some ()) } }
};
}
