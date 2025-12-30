// Generated macro for is_entrypoint_fn (function)
macro_rules! Depcrateis_entrypoint_fn {
() => {
// Module: crate
// Provides: {"is_entrypoint_fn"}
// Dependencies: {}
# [doc = " Returns `true` if the provided `def_id` is an entrypoint to a program."] pub fn is_entrypoint_fn (cx : & LateContext < '_ > , def_id : DefId) -> bool { cx . tcx . entry_fn (()) . is_some_and (| (entry_fn_def_id , _) | def_id == entry_fn_def_id) }
};
}
