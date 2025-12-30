// Generated macro for impl_571 (impl)
macro_rules! Depcrate_inferimpl_571 {
() => {
// Module: crate::infer
// Provides: {"impl_571"}
// Dependencies: {}
impl < 'tcx > TypeTrace < 'tcx > { pub fn span (& self) -> Span { self . cause . span } pub fn types (cause : & ObligationCause < 'tcx > , a : Ty < 'tcx > , b : Ty < 'tcx >) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) , } } pub fn trait_refs (cause : & ObligationCause < 'tcx > , a : ty :: TraitRef < 'tcx > , b : ty :: TraitRef < 'tcx > ,) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: TraitRefs (ExpectedFound :: new (a , b)) } } pub fn consts (cause : & ObligationCause < 'tcx > , a : ty :: Const < 'tcx > , b : ty :: Const < 'tcx > ,) -> TypeTrace < 'tcx > { TypeTrace { cause : cause . clone () , values : ValuePairs :: Terms (ExpectedFound :: new (a . into () , b . into ())) , } } }
};
}
