// Generated macro for check_type_wf (function)
macro_rules! Depcrate_check_wfcheckcheck_type_wf {
() => {
// Module: crate::check::wfcheck
// Provides: {"check_type_wf"}
// Dependencies: {}
pub (super) fn check_type_wf (tcx : TyCtxt < '_ > , () : ()) -> Result < () , ErrorGuaranteed > { let items = tcx . hir_crate_items (()) ; let res = items . par_items (| item | tcx . ensure_ok () . check_well_formed (item . owner_id . def_id)) . and (items . par_impl_items (| item | tcx . ensure_ok () . check_well_formed (item . owner_id . def_id))) . and (items . par_trait_items (| item | tcx . ensure_ok () . check_well_formed (item . owner_id . def_id))) . and (items . par_foreign_items (| item | tcx . ensure_ok () . check_well_formed (item . owner_id . def_id)) ,) . and (items . par_nested_bodies (| item | tcx . ensure_ok () . check_well_formed (item))) . and (items . par_opaques (| item | tcx . ensure_ok () . check_well_formed (item))) ; super :: entry :: check_for_entry_fn (tcx) ; res }
};
}
