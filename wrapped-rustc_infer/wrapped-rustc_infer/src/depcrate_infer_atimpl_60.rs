// Generated macro for impl_60 (impl)
macro_rules! Depcrate_infer_atimpl_60 {
() => {
// Module: crate::infer::at
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: TraitRef < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: TraitRefs (ExpectedFound :: new (a , b)) } } }
};
}
