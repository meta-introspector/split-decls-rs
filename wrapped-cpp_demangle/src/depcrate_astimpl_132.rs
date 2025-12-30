// Generated macro for impl_132 (impl)
macro_rules! Depcrate_astimpl_132 {
() => {
// Module: crate::ast
// Provides: {"impl_132"}
// Dependencies: {}
impl IsCtorDtorConversion for UnqualifiedName { fn is_ctor_dtor_conversion (& self , _ : & SubstitutionTable) -> bool { match * self { UnqualifiedName :: CtorDtor (..) | UnqualifiedName :: Operator (OperatorName :: Conversion (_) , _) => true , UnqualifiedName :: Operator (..) | UnqualifiedName :: Source (..) | UnqualifiedName :: LocalSourceName (..) | UnqualifiedName :: UnnamedType (..) | UnqualifiedName :: ClosureType (..) => false , } } }
};
}
