// Generated macro for impl_58 (impl)
macro_rules! Depcrate_infer_atimpl_58 {
() => {
// Module: crate::infer::at
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ty :: GenericArg < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : match (a . kind () , b . kind ()) { (GenericArgKind :: Lifetime (a) , GenericArgKind :: Lifetime (b)) => { ValuePairs :: Regions (ExpectedFound :: new (a , b)) } (GenericArgKind :: Type (a) , GenericArgKind :: Type (b)) => { ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) } (GenericArgKind :: Const (a) , GenericArgKind :: Const (b)) => { ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) } _ => bug ! ("relating different kinds: {a:?} {b:?}") , } , } } }
};
}
