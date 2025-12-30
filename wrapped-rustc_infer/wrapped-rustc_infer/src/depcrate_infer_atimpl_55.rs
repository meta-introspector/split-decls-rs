// Generated macro for impl_55 (impl)
macro_rules! Depcrate_infer_atimpl_55 {
() => {
// Module: crate::infer::at
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for Ty < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) , } } }
};
}
