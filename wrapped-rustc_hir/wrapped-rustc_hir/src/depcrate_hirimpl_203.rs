// Generated macro for impl_203 (impl)
macro_rules! Depcrate_hirimpl_203 {
() => {
// Module: crate::hir
// Provides: {"impl_203"}
// Dependencies: {}
impl ParamName { pub fn ident (& self) -> Ident { match * self { ParamName :: Plain (ident) | ParamName :: Error (ident) => ident , ParamName :: Fresh => Ident :: with_dummy_span (kw :: UnderscoreLifetime) , } } }
};
}
