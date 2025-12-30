// Generated macro for impl_9365 (impl)
macro_rules! Depcrate_redundant_pub_crateimpl_9365 {
() => {
// Module: crate::redundant_pub_crate
// Provides: {"impl_9365"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for RedundantPubCrate { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if cx . tcx . visibility (item . owner_id . def_id) == ty :: Visibility :: Restricted (CRATE_DEF_ID . to_def_id ()) && ! cx . effective_visibilities . is_exported (item . owner_id . def_id) && self . is_exported . last () == Some (& false) && ! is_ignorable_export (item) && ! item . span . in_external_macro (cx . sess () . source_map ()) { let span = item . kind . ident () . map_or (item . span , | ident | item . span . with_hi (ident . span . hi ())) ; let descr = cx . tcx . def_kind (item . owner_id) . descr (item . owner_id . to_def_id ()) ; span_lint_and_then (cx , REDUNDANT_PUB_CRATE , span , format ! ("pub(crate) {descr} inside private module") , | diag | { diag . span_suggestion (item . vis_span , "consider using" , "pub" . to_string () , Applicability :: MachineApplicable ,) ; } ,) ; } if let ItemKind :: Mod { .. } = item . kind { self . is_exported . push (cx . effective_visibilities . is_exported (item . owner_id . def_id)) ; } } fn check_item_post (& mut self , _cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if let ItemKind :: Mod { .. } = item . kind { self . is_exported . pop () . expect ("unbalanced check_item/check_item_post") ; } } }
};
}
