// Generated macro for check_trait_item (function)
macro_rules! Depcrate_functions_must_usecheck_trait_item {
() => {
// Module: crate::functions::must_use
// Provides: {"check_trait_item"}
// Dependencies: {}
pub (super) fn check_trait_item < 'tcx > (cx : & LateContext < 'tcx > , item : & 'tcx hir :: TraitItem < '_ >) { if let hir :: TraitItemKind :: Fn (ref sig , ref eid) = item . kind { let is_public = cx . effective_visibilities . is_exported (item . owner_id . def_id) ; let fn_header_span = item . span . with_hi (sig . decl . output . span () . hi ()) ; let attrs = cx . tcx . hir_attrs (item . hir_id ()) ; let attr = find_attr ! (cx . tcx . hir_attrs (item . hir_id ()) , AttributeKind :: MustUse { span , reason } => (span , reason)) ; if let Some ((attr_span , reason)) = attr { check_needless_must_use (cx , sig . decl , item . owner_id , item . span , fn_header_span , * attr_span , * reason , attrs , sig ,) ; } else if let hir :: TraitFn :: Provided (eid) = * eid { let body = cx . tcx . hir_body (eid) ; if attr . is_none () && is_public && ! is_proc_macro (attrs) { check_must_use_candidate (cx , sig . decl , body , item . span , item . ident . span , item . owner_id , "this method could have a `#[must_use]` attribute" ,) ; } } } }
};
}
