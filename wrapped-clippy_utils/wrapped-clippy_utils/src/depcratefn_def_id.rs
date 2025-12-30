// Generated macro for fn_def_id (function)
macro_rules! Depcratefn_def_id {
() => {
// Module: crate
// Provides: {"fn_def_id"}
// Dependencies: {}
# [doc = " Returns the `DefId` of the callee if the given expression is a function or method call."] pub fn fn_def_id (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < DefId > { fn_def_id_with_node_args (cx , expr) . map (| (did , _) | did) }
};
}
