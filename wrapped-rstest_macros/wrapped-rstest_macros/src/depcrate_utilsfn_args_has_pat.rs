// Generated macro for fn_args_has_pat (function)
macro_rules! Depcrate_utilsfn_args_has_pat {
() => {
// Module: crate::utils
// Provides: {"fn_args_has_pat"}
// Dependencies: {}
# [doc = " Return if function declaration has an ident"] # [doc = ""] pub (crate) fn fn_args_has_pat (fn_decl : & ItemFn , pat : & Pat) -> bool { fn_args_pats (fn_decl) . any (| id | compare_pat (id , pat)) }
};
}
