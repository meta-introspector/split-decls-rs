// Generated macro for impl_645 (impl)
macro_rules! Depcrate_lifetime_syntaximpl_645 {
() => {
// Module: crate::lifetime_syntax
// Provides: {"impl_645"}
// Dependencies: {}
impl LifetimeSyntaxCategory { fn new (syntax_source : (hir :: LifetimeSyntax , LifetimeSource)) -> Option < Self > { use LifetimeSource :: * ; use hir :: LifetimeSyntax :: * ; match syntax_source { (Implicit , Reference) | (ExplicitAnonymous , Reference) | (ExplicitAnonymous , Path { .. }) | (ExplicitAnonymous , OutlivesBound | PreciseCapturing) => { Some (Self :: Elided) } (Implicit , Path { .. }) => { Some (Self :: Hidden) } (ExplicitBound , Reference) | (ExplicitBound , Path { .. }) | (ExplicitBound , OutlivesBound | PreciseCapturing) => { Some (Self :: Named) } (Implicit , OutlivesBound | PreciseCapturing) | (_ , Other) => { None } } } }
};
}
