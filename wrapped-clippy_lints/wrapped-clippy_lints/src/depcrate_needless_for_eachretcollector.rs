// Generated macro for RetCollector (struct)
macro_rules! Depcrate_needless_for_eachRetCollector {
() => {
// Module: crate::needless_for_each
// Provides: {"RetCollector"}
// Dependencies: {}
# [doc = " This type plays two roles."] # [doc = " 1. Collect spans of `return` in the closure body."] # [doc = " 2. Detect use of `return` in `Loop` in the closure body."] # [doc = ""] # [doc = " NOTE: The functionality of this type is similar to"] # [doc = " [`clippy_utils::visitors::find_all_ret_expressions`], but we can't use"] # [doc = " `find_all_ret_expressions` instead of this type. The reasons are:"] # [doc = " 1. `find_all_ret_expressions` passes the argument of `ExprKind::Ret` to a callback, but what we"] # [doc = "    need here is `ExprKind::Ret` itself."] # [doc = " 2. We can't trace current loop depth with `find_all_ret_expressions`."] # [derive (Default)] struct RetCollector { spans : Vec < Span > , ret_in_loop : bool , loop_depth : u16 , }
};
}
