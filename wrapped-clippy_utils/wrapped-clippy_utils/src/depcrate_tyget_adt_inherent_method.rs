// Generated macro for get_adt_inherent_method (function)
macro_rules! Depcrate_tyget_adt_inherent_method {
() => {
// Module: crate::ty
// Provides: {"get_adt_inherent_method"}
// Dependencies: {}
# [doc = " Checks if a Ty<'_> has some inherent method Symbol."] # [doc = ""] # [doc = " This does not look for impls in the type's `Deref::Target` type."] # [doc = " If you need this, you should wrap this call in `clippy_utils::ty::deref_chain().any(...)`."] pub fn get_adt_inherent_method < 'a > (cx : & 'a LateContext < '_ > , ty : Ty < '_ > , method_name : Symbol) -> Option < & 'a AssocItem > { if let Some (ty_did) = ty . ty_adt_def () . map (AdtDef :: did) { cx . tcx . inherent_impls (ty_did) . iter () . find_map (| & did | { cx . tcx . associated_items (did) . filter_by_name_unhygienic (method_name) . next () . filter (| item | item . as_tag () == AssocTag :: Fn) }) } else { None } }
};
}
