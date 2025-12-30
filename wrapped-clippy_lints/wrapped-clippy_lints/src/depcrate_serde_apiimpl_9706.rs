// Generated macro for impl_9706 (impl)
macro_rules! Depcrate_serde_apiimpl_9706 {
() => {
// Module: crate::serde_api
// Provides: {"impl_9706"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SerdeApi { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , items , .. }) = item . kind { let did = of_trait . trait_ref . path . res . def_id () ; if paths :: SERDE_DE_VISITOR . matches (cx , did) { let mut seen_str = None ; let mut seen_string = None ; for item in items { match cx . tcx . item_name (item . owner_id) { sym :: visit_str => seen_str = Some (cx . tcx . def_span (item . owner_id)) , sym :: visit_string => seen_string = Some (cx . tcx . def_span (item . owner_id)) , _ => { } , } } if let Some (span) = seen_string && seen_str . is_none () { span_lint (cx , SERDE_API_MISUSE , span , "you should not implement `visit_string` without also implementing `visit_str`" ,) ; } } } } }
};
}
