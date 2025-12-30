// Generated macro for check (function)
macro_rules! Depcrate_attrs_inline_alwayscheck {
() => {
// Module: crate::attrs::inline_always
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , span : Span , name : Symbol , attrs : & [Attribute]) { if span . from_expansion () { return ; } if let Some (span) = find_attr ! (attrs , AttributeKind :: Inline (InlineAttr :: Always , span) => * span) { span_lint (cx , INLINE_ALWAYS , span , format ! ("you have declared `#[inline(always)]` on `{name}`. This is usually a bad idea") ,) ; } }
};
}
