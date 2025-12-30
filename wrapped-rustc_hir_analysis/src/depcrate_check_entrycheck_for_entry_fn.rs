// Generated macro for check_for_entry_fn (function)
macro_rules! Depcrate_check_entrycheck_for_entry_fn {
() => {
// Module: crate::check::entry
// Provides: {"check_for_entry_fn"}
// Dependencies: {}
pub (crate) fn check_for_entry_fn (tcx : TyCtxt < '_ >) { match tcx . entry_fn (()) { Some ((def_id , EntryFnType :: Main { .. })) => check_main_fn_ty (tcx , def_id) , _ => { } } }
};
}
