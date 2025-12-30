// Generated macro for maybe_stage_features (function)
macro_rules! Depcrate_feature_gatemaybe_stage_features {
() => {
// Module: crate::feature_gate
// Provides: {"maybe_stage_features"}
// Dependencies: {}
fn maybe_stage_features (sess : & Session , features : & Features , krate : & ast :: Crate) { if sess . opts . unstable_features . is_nightly_build () { return ; } if features . enabled_features () . is_empty () { return ; } let mut errored = false ; for attr in krate . attrs . iter () . filter (| attr | attr . has_name (sym :: feature)) { let mut err = errors :: FeatureOnNonNightly { span : attr . span , channel : option_env ! ("CFG_RELEASE_CHANNEL") . unwrap_or ("(unknown)") , stable_features : vec ! [] , sugg : None , } ; let mut all_stable = true ; for ident in attr . meta_item_list () . into_iter () . flatten () . flat_map (| nested | nested . ident ()) { let name = ident . name ; let stable_since = features . enabled_lang_features () . iter () . find (| feat | feat . gate_name == name) . map (| feat | feat . stable_since) . flatten () ; if let Some (since) = stable_since { err . stable_features . push (errors :: StableFeature { name , since }) ; } else { all_stable = false ; } } if all_stable { err . sugg = Some (attr . span) ; } sess . dcx () . emit_err (err) ; errored = true ; } assert ! (errored) ; }
};
}
