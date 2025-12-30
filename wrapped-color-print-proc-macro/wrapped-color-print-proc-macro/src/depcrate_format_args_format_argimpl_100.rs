// Generated macro for impl_100 (impl)
macro_rules! Depcrate_format_args_format_argimpl_100 {
() => {
// Module: crate::format_args::format_arg
// Provides: {"impl_100"}
// Dependencies: {}
impl ToTokens for FormatArg { fn to_tokens (& self , tokens : & mut TokenStream2) { if let Some ((arg_name , eq)) = & self . arg_name { arg_name . to_tokens (tokens) ; eq . to_tokens (tokens) ; } self . expr . to_tokens (tokens) ; } }
};
}
