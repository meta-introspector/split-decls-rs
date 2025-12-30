// Generated macro for get_function_kind (function)
macro_rules! Depcrate_concurrency_genmc_schedulingget_function_kind {
() => {
// Module: crate::concurrency::genmc::scheduling
// Provides: {"get_function_kind"}
// Dependencies: {}
fn get_function_kind < 'tcx > (ecx : & InterpCx < 'tcx , MiriMachine < 'tcx > > , func_ty : Ty < 'tcx > ,) -> InterpResult < 'tcx , NextInstrKind > { use NextInstrKind :: * ; let callee_def_id = match func_ty . kind () { ty :: FnDef (def_id , _args) => * def_id , _ => return interp_ok (MaybeAtomic (ActionKind :: Load)) , } ; let Some (intrinsic_def) = ecx . tcx . intrinsic (callee_def_id) else { if ecx . tcx . is_foreign_item (callee_def_id) { return interp_ok (MaybeAtomic (ActionKind :: Load)) ; } if ecx . tcx . is_diagnostic_item (rustc_span :: sym :: sys_mutex_lock , callee_def_id) || ecx . tcx . is_diagnostic_item (rustc_span :: sym :: sys_mutex_try_lock , callee_def_id) { return interp_ok (MaybeAtomic (ActionKind :: Load)) ; } else if ecx . tcx . is_diagnostic_item (rustc_span :: sym :: sys_mutex_unlock , callee_def_id) { return interp_ok (MaybeAtomic (ActionKind :: NonLoad)) ; } return interp_ok (NonAtomic) ; } ; let intrinsic_name = intrinsic_def . name . as_str () ; let Some (suffix) = intrinsic_name . strip_prefix ("atomic_") else { return interp_ok (NonAtomic) ; } ; interp_ok (MaybeAtomic (if matches ! (suffix , "store" | "fence" | "singlethreadfence") { ActionKind :: NonLoad } else { ActionKind :: Load })) }
};
}
