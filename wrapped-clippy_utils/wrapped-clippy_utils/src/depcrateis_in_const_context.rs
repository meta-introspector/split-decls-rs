// Generated macro for is_in_const_context (function)
macro_rules! Depcrateis_in_const_context {
() => {
// Module: crate
// Provides: {"is_in_const_context"}
// Dependencies: {}
# [doc = " Checks if we are currently in a const context (e.g. `const fn`, `static`/`const` initializer)."] # [doc = ""] # [doc = " The current context is determined based on the current body which is set before calling a lint's"] # [doc = " entry point (any function on `LateLintPass`). If you need to check in a different context use"] # [doc = " `tcx.hir_is_inside_const_context(_)`."] # [doc = ""] # [doc = " Do not call this unless the `LateContext` has an enclosing body. For release build this case"] # [doc = " will safely return `false`, but debug builds will ICE. Note that `check_expr`, `check_block`,"] # [doc = " `check_pat` and a few other entry points will always have an enclosing body. Some entry points"] # [doc = " like `check_path` or `check_ty` may or may not have one."] pub fn is_in_const_context (cx : & LateContext < '_ >) -> bool { debug_assert ! (cx . enclosing_body . is_some () , "`LateContext` has no enclosing body") ; cx . enclosing_body . is_some_and (| id | { cx . tcx . hir_body_const_context (cx . tcx . hir_body_owner_def_id (id)) . is_some () }) }
};
}
