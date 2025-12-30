// Generated macro for impl_321 (impl)
macro_rules! Depcrate_hirimpl_321 {
() => {
// Module: crate::hir
// Provides: {"impl_321"}
// Dependencies: {}
impl < 'hir > QPath < 'hir > { # [doc = " Returns the span of this `QPath`."] pub fn span (& self) -> Span { match * self { QPath :: Resolved (_ , path) => path . span , QPath :: TypeRelative (qself , ps) => qself . span . to (ps . ident . span) , QPath :: LangItem (_ , span) => span , } } # [doc = " Returns the span of the qself of this `QPath`. For example, `()` in"] # [doc = " `<() as Trait>::method`."] pub fn qself_span (& self) -> Span { match * self { QPath :: Resolved (_ , path) => path . span , QPath :: TypeRelative (qself , _) => qself . span , QPath :: LangItem (_ , span) => span , } } }
};
}
