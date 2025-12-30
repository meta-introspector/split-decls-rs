// Generated macro for impl_63 (impl)
macro_rules! Depcrate_infer_atimpl_63 {
() => {
// Module: crate::infer::at
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: FnSig < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: PolySigs (ExpectedFound :: new (ty :: Binder :: dummy (a) , ty :: Binder :: dummy (b) ,)) , } } }
};
}
