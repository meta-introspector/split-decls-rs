// Generated macro for impl_2088 (impl)
macro_rules! Depcrate_exhaustive_itemsimpl_2088 {
() => {
// Module: crate::exhaustive_items
// Provides: {"impl_2088"}
// Dependencies: {}
impl LateLintPass < '_ > for ExhaustiveItems { fn check_item (& mut self , cx : & LateContext < '_ > , item : & Item < '_ >) { let (lint , msg , fields) = match item . kind { ItemKind :: Enum (..) => (EXHAUSTIVE_ENUMS , "exported enums should not be exhaustive" , [] . as_slice () ,) , ItemKind :: Struct (_ , _ , v) if v . fields () . iter () . all (| f | f . default . is_none ()) => (EXHAUSTIVE_STRUCTS , "exported structs should not be exhaustive" , v . fields () ,) , _ => return , } ; if cx . effective_visibilities . is_exported (item . owner_id . def_id) && let attrs = cx . tcx . hir_attrs (item . hir_id ()) && ! find_attr ! (attrs , AttributeKind :: NonExhaustive (..)) && fields . iter () . all (| f | cx . tcx . visibility (f . def_id) . is_public ()) { span_lint_and_then (cx , lint , item . span , msg , | diag | { let suggestion_span = item . span . shrink_to_lo () ; let indent = " " . repeat (indent_of (cx , item . span) . unwrap_or (0)) ; let sugg = format ! ("#[non_exhaustive]\n{indent}") ; diag . span_suggestion_verbose (suggestion_span , "try adding #[non_exhaustive]" , sugg , Applicability :: MaybeIncorrect ,) ; }) ; } } }
};
}
