// Generated macro for check_missing_inline_attrs (function)
macro_rules! Depcrate_missing_inlinecheck_missing_inline_attrs {
() => {
// Module: crate::missing_inline
// Provides: {"check_missing_inline_attrs"}
// Dependencies: {}
fn check_missing_inline_attrs (cx : & LateContext < '_ > , attrs : & [Attribute] , sp : Span , desc : & 'static str , hir_id : Option < hir :: HirId > ,) { if ! find_attr ! (attrs , AttributeKind :: Inline (..)) { let msg = format ! ("missing `#[inline]` for {desc}") ; if let Some (hir_id) = hir_id { span_lint_hir (cx , MISSING_INLINE_IN_PUBLIC_ITEMS , hir_id , sp , msg) ; } else { span_lint (cx , MISSING_INLINE_IN_PUBLIC_ITEMS , sp , msg) ; } } }
};
}
