// Generated macro for initializer_can_be_made_const (function)
macro_rules! Depcrate_missing_const_for_thread_localinitializer_can_be_made_const {
() => {
// Module: crate::missing_const_for_thread_local
// Provides: {"initializer_can_be_made_const"}
// Dependencies: {}
# [inline] fn initializer_can_be_made_const (cx : & LateContext < '_ > , defid : rustc_span :: def_id :: DefId , msrv : Msrv) -> bool { if ! fn_has_unsatisfiable_preds (cx , defid) && let mir = cx . tcx . optimized_mir (defid) && let Ok (()) = is_min_const_fn (cx , mir , msrv) { return true ; } false }
};
}
