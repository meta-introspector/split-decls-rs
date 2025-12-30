// Generated macro for impl_68 (impl)
macro_rules! Depcrate_infer_atimpl_68 {
() => {
// Module: crate::infer::at
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: ExistentialProjection < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: ExistentialProjection (ExpectedFound :: new (ty :: Binder :: dummy (a) , ty :: Binder :: dummy (b) ,)) , } } }
};
}
