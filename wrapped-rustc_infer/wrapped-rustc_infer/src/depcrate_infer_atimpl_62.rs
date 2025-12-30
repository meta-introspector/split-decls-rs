// Generated macro for impl_62 (impl)
macro_rules! Depcrate_infer_atimpl_62 {
() => {
// Module: crate::infer::at
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: AliasTerm < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Aliases (ExpectedFound :: new (a , b)) } } }
};
}
