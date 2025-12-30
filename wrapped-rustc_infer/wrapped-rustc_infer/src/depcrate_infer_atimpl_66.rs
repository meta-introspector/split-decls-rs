// Generated macro for impl_66 (impl)
macro_rules! Depcrate_infer_atimpl_66 {
() => {
// Module: crate::infer::at
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: ExistentialTraitRef < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: ExistentialTraitRef (ExpectedFound :: new (ty :: Binder :: dummy (a) , ty :: Binder :: dummy (b) ,)) , } } }
};
}
