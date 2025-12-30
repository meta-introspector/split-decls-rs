// Generated macro for impl_61 (impl)
macro_rules! Depcrate_infer_atimpl_61 {
() => {
// Module: crate::infer::at
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: AliasTy < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Aliases (ExpectedFound :: new (a . into () , b . into ())) , } } }
};
}
