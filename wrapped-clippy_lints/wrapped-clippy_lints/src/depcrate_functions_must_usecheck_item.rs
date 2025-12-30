// Generated macro for check_item (function)
macro_rules! Depcrate_functions_must_usecheck_item {
() => {
// Module: crate::functions::must_use
// Provides: {"check_item"}
// Dependencies: {}
pub (super) fn check_item < 'tcx > (cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < '_ >) { let attrs = cx . tcx . hir_attrs (item . hir_id ()) ; let attr = find_attr ! (cx . tcx . hir_attrs (item . hir_id ()) , AttributeKind :: MustUse { span , reason } => (span , reason)) ; if let hir :: ItemKind :: Fn { ref sig , body : ref body_id , ident , .. } = item . kind { let is_public = cx . effective_visibilities . is_exported (item . owner_id . def_id) ; let fn_header_span = item . span . with_hi (sig . decl . output . span () . hi ()) ; if let Some ((attr_span , reason)) = attr { check_needless_must_use (cx , sig . decl , item . owner_id , item . span , fn_header_span , * attr_span , * reason , attrs , sig ,) ; } else if is_public && ! is_proc_macro (attrs) && ! find_attr ! (attrs , AttributeKind :: NoMangle (..)) { check_must_use_candidate (cx , sig . decl , cx . tcx . hir_body (* body_id) , item . span , ident . span , item . owner_id , "this function could have a `#[must_use]` attribute" ,) ; } } }
};
}
