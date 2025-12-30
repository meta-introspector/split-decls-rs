// Generated macro for impl_9080 (impl)
macro_rules! Depcrate_pub_underscore_fieldsimpl_9080 {
() => {
// Module: crate::pub_underscore_fields
// Provides: {"impl_9080"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PubUnderscoreFields { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { let ItemKind :: Struct (_ , _ , variant_data) = & item . kind else { return ; } ; let is_visible = | field : & FieldDef < '_ > | match self . behavior { PubUnderscoreFieldsBehaviour :: PubliclyExported => cx . effective_visibilities . is_reachable (field . def_id) , PubUnderscoreFieldsBehaviour :: AllPubFields => { ! field . vis_span . is_empty () } , } ; for field in variant_data . fields () { if field . ident . as_str () . starts_with ('_') && is_visible (field) && ! is_doc_hidden (cx . tcx . hir_attrs (field . hir_id)) && ! field . ty . basic_res () . is_lang_item (cx , LangItem :: PhantomData) { span_lint_hir_and_then (cx , PUB_UNDERSCORE_FIELDS , field . hir_id , field . vis_span . to (field . ident . span) , "field marked as public but also inferred as unused because it's prefixed with `_`" , | diag | { diag . help ("consider removing the underscore, or making the field private") ; } ,) ; } } } }
};
}
