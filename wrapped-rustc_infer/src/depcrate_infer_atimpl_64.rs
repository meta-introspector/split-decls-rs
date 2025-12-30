// Generated macro for impl_64 (impl)
macro_rules! Depcrate_infer_atimpl_64 {
() => {
// Module: crate::infer::at
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: PolyFnSig < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: PolySigs (ExpectedFound :: new (a , b)) } } }
};
}
