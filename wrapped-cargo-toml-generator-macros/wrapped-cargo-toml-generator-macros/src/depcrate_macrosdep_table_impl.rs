// Generated macro for dep_table_impl (function)
macro_rules! Depcrate_macrosdep_table_impl {
() => {
// Module: crate::macros
// Provides: {"dep_table_impl"}
// Dependencies: {}
pub fn dep_table_impl (input : TokenStream) -> TokenStream { let DepTableInput { name , table_content , .. } = parse_macro_input ! (input as DepTableInput) ; let name_ident = Ident :: new (& name . value () , name . span ()) ; quote ! { # name_ident = { # table_content } } . into () }
};
}
