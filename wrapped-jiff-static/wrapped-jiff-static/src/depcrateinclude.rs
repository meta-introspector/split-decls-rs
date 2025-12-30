// Generated macro for include (function)
macro_rules! Depcrateinclude {
() => {
// Module: crate
// Provides: {"include"}
// Dependencies: {}
# [proc_macro] pub fn include (input : TokenStream) -> TokenStream { let input = syn :: parse_macro_input ! (input as Include) ; proc_macro :: TokenStream :: from (input . quote ()) }
};
}
