// Generated macro for __rewrite_self_param (macro)
macro_rules! Depcrate___macros_rewrite_self_param__rewrite_self_param {
() => {
// Module: crate::__macros::rewrite_self_param
// Provides: {"__rewrite_self_param"}
// Dependencies: {}
# [doc = " Detect instance vs. class method."] # [doc (hidden)] # [macro_export] macro_rules ! __rewrite_self_param { { ($ ($ params : tt) *) ($ out_macro : path) $ ($ out_args : tt) * } => { $ crate :: __rewrite_self_param_inner ! { ($ ($ params) *) ($ ($ params) *) ($ out_macro) $ ($ out_args) * } } ; }
};
}
