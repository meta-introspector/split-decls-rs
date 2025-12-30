// Generated macro for lint_item_shadowing_supertrait_item (function)
macro_rules! Depcrate_check_wfchecklint_item_shadowing_supertrait_item {
() => {
// Module: crate::check::wfcheck
// Provides: {"lint_item_shadowing_supertrait_item"}
// Dependencies: {}
fn lint_item_shadowing_supertrait_item < 'tcx > (tcx : TyCtxt < 'tcx > , trait_item_def_id : LocalDefId) { let item_name = tcx . item_name (trait_item_def_id . to_def_id ()) ; let trait_def_id = tcx . local_parent (trait_item_def_id) ; let shadowed : Vec < _ > = traits :: supertrait_def_ids (tcx , trait_def_id . to_def_id ()) . skip (1) . flat_map (| supertrait_def_id | { tcx . associated_items (supertrait_def_id) . filter_by_name_unhygienic (item_name) }) . collect () ; if ! shadowed . is_empty () { let shadowee = if let [shadowed] = shadowed [..] { errors :: SupertraitItemShadowee :: Labeled { span : tcx . def_span (shadowed . def_id) , supertrait : tcx . item_name (shadowed . trait_container (tcx) . unwrap ()) , } } else { let (traits , spans) : (Vec < _ > , Vec < _ >) = shadowed . iter () . map (| item | { (tcx . item_name (item . trait_container (tcx) . unwrap ()) , tcx . def_span (item . def_id)) }) . unzip () ; errors :: SupertraitItemShadowee :: Several { traits : traits . into () , spans : spans . into () } } ; tcx . emit_node_span_lint (SUPERTRAIT_ITEM_SHADOWING_DEFINITION , tcx . local_def_id_to_hir_id (trait_item_def_id) , tcx . def_span (trait_item_def_id) , errors :: SupertraitItemShadowing { item : item_name , subtrait : tcx . item_name (trait_def_id . to_def_id ()) , shadowee , } ,) ; } }
};
}
