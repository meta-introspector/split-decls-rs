// Generated macro for path_to_local_id (function)
macro_rules! Depcratepath_to_local_id {
() => {
// Module: crate
// Provides: {"path_to_local_id"}
// Dependencies: {}
# [doc = " Returns true if the expression is a path to a local with the specified `HirId`."] # [doc = " Use this function to see if an expression matches a function argument or a match binding."] pub fn path_to_local_id (expr : & Expr < '_ > , id : HirId) -> bool { path_to_local (expr) == Some (id) }
};
}
