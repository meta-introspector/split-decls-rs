// Generated macro for impl_59 (impl)
macro_rules! Depcrate_infer_atimpl_59 {
() => {
// Module: crate::infer::at
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: Term < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Terms (ExpectedFound :: new (a , b)) } } }
};
}
