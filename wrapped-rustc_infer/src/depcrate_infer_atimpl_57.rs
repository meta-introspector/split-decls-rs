// Generated macro for impl_57 (impl)
macro_rules! Depcrate_infer_atimpl_57 {
() => {
// Module: crate::infer::at
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for Const < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) , } } }
};
}
