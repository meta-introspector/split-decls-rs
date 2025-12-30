// Generated macro for impl_56 (impl)
macro_rules! Depcrate_infer_atimpl_56 {
() => {
// Module: crate::infer::at
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: Region < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Regions (ExpectedFound :: new (a , b)) } } }
};
}
