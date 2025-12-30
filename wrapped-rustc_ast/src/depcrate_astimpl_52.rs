// Generated macro for impl_52 (impl)
macro_rules! Depcrate_astimpl_52 {
() => {
// Module: crate::ast
// Provides: {"impl_52"}
// Dependencies: {}
impl GenericBound { pub fn span (& self) -> Span { match self { GenericBound :: Trait (t , ..) => t . span , GenericBound :: Outlives (l) => l . ident . span , GenericBound :: Use (_ , span) => * span , } } }
};
}
