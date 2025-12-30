// Generated macro for check_trait (function)
macro_rules! Depcrate_check_wfcheckcheck_trait {
() => {
// Module: crate::check::wfcheck
// Provides: {"check_trait"}
// Dependencies: {}
# [instrument (skip (tcx , item))] fn check_trait (tcx : TyCtxt < '_ > , item : & hir :: Item < '_ >) -> Result < () , ErrorGuaranteed > { debug ! (? item . owner_id) ; let def_id = item . owner_id . def_id ; if tcx . is_lang_item (def_id . into () , LangItem :: PointeeSized) { return Ok (()) ; } let trait_def = tcx . trait_def (def_id) ; if trait_def . is_marker || matches ! (trait_def . specialization_kind , TraitSpecializationKind :: Marker) { for associated_def_id in & * tcx . associated_item_def_ids (def_id) { struct_span_code_err ! (tcx . dcx () , tcx . def_span (* associated_def_id) , E0714 , "marker traits cannot have associated items" ,) . emit () ; } } let res = enter_wf_checking_ctxt (tcx , def_id , | wfcx | { check_where_clauses (wfcx , def_id) ; Ok (()) }) ; if let hir :: ItemKind :: Trait (..) = item . kind { check_gat_where_clauses (tcx , item . owner_id . def_id) ; } res }
};
}
