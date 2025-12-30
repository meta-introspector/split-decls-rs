// Generated macro for impl_105 (impl)
macro_rules! Depcrate_codegen_errorimpl_105 {
() => {
// Module: crate::codegen::error
// Provides: {"impl_105"}
// Dependencies: {}
impl ToTokens for ErrorCheck < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let at_call = if let Some (ref s) = self . location { quote ! (. map_err (| e | e . at (# s))) } else { quote ! () } ; tokens . append_all (quote ! { __errors . finish () # at_call ?; }) } }
};
}
