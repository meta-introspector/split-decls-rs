// Generated macro for impl_203 (impl)
macro_rules! Depcrate_codegen_postfix_transformimpl_203 {
() => {
// Module: crate::codegen::postfix_transform
// Provides: {"impl_203"}
// Dependencies: {}
impl ToTokens for PostfixTransform { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let Self { transformer , function , } = self ; tokens . append_all (quote ! (.# transformer (# function))) } }
};
}
