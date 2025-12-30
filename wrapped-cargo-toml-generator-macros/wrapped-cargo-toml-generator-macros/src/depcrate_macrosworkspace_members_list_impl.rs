// Generated macro for workspace_members_list_impl (function)
macro_rules! Depcrate_macrosworkspace_members_list_impl {
() => {
// Module: crate::macros
// Provides: {"workspace_members_list_impl"}
// Dependencies: {}
pub fn workspace_members_list_impl (input : TokenStream) -> TokenStream { let parsed : Punctuated < LitStr , Token ! [,] > = parse_macro_input ! (input with Punctuated :: parse_terminated) ; let expanded = quote ! { [# parsed] } ; expanded . into () }
};
}
