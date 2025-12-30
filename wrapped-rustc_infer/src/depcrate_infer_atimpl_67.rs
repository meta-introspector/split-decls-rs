// Generated macro for impl_67 (impl)
macro_rules! Depcrate_infer_atimpl_67 {
() => {
// Module: crate::infer::at
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: PolyExistentialProjection < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: ExistentialProjection (ExpectedFound :: new (a , b)) , } } }
};
}
