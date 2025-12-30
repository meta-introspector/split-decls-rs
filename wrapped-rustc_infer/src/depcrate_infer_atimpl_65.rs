// Generated macro for impl_65 (impl)
macro_rules! Depcrate_infer_atimpl_65 {
() => {
// Module: crate::infer::at
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: PolyExistentialTraitRef < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: ExistentialTraitRef (ExpectedFound :: new (a , b)) , } } }
};
}
