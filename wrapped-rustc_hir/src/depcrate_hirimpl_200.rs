// Generated macro for impl_200 (impl)
macro_rules! Depcrate_hirimpl_200 {
() => {
// Module: crate::hir
// Provides: {"impl_200"}
// Dependencies: {}
impl From < Ident > for LifetimeSyntax { fn from (ident : Ident) -> Self { let name = ident . name ; if name == sym :: empty { unreachable ! ("A lifetime name should never be empty") ; } else if name == kw :: UnderscoreLifetime { LifetimeSyntax :: ExplicitAnonymous } else { debug_assert ! (name . as_str () . starts_with ('\'')) ; LifetimeSyntax :: ExplicitBound } } }
};
}
