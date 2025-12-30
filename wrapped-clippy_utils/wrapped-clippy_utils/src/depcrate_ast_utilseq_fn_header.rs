// Generated macro for eq_fn_header (function)
macro_rules! Depcrate_ast_utilseq_fn_header {
() => {
// Module: crate::ast_utils
// Provides: {"eq_fn_header"}
// Dependencies: {}
pub fn eq_fn_header (l : & FnHeader , r : & FnHeader) -> bool { matches ! (l . safety , Safety :: Default) == matches ! (r . safety , Safety :: Default) && eq_opt_coroutine_kind (l . coroutine_kind , r . coroutine_kind) && matches ! (l . constness , Const :: No) == matches ! (r . constness , Const :: No) && eq_ext (& l . ext , & r . ext) }
};
}
