// Generated macro for determine_parameters_to_be_inferred (function)
macro_rules! Depcrate_variance_termsdetermine_parameters_to_be_inferred {
() => {
// Module: crate::variance::terms
// Provides: {"determine_parameters_to_be_inferred"}
// Dependencies: {}
pub (crate) fn determine_parameters_to_be_inferred < 'a , 'tcx > (tcx : TyCtxt < 'tcx > , arena : & 'a DroplessArena ,) -> TermsContext < 'a , 'tcx > { let mut terms_cx = TermsContext { tcx , arena , inferred_starts : Default :: default () , inferred_terms : vec ! [] , lang_items : lang_items (tcx) , } ; let crate_items = tcx . hir_crate_items (()) ; for def_id in crate_items . definitions () { debug ! ("add_inferreds for item {:?}" , def_id) ; let def_kind = tcx . def_kind (def_id) ; match def_kind { DefKind :: Struct | DefKind :: Union | DefKind :: Enum => { terms_cx . add_inferreds_for_item (def_id) ; let adt = tcx . adt_def (def_id) ; for variant in adt . variants () { if let Some (ctor_def_id) = variant . ctor_def_id () { terms_cx . add_inferreds_for_item (ctor_def_id . expect_local ()) ; } } } DefKind :: Fn | DefKind :: AssocFn => terms_cx . add_inferreds_for_item (def_id) , DefKind :: TyAlias if tcx . type_alias_is_lazy (def_id) => { terms_cx . add_inferreds_for_item (def_id) } _ => { } } } terms_cx }
};
}
