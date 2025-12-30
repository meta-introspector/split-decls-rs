// Generated macro for get (function)
macro_rules! Depcrateget {
() => {
// Module: crate
// Provides: {"get"}
// Dependencies: {}
# [cfg (feature = "tzdb")] # [proc_macro] pub fn get (input : TokenStream) -> TokenStream { let input = syn :: parse_macro_input ! (input as Get) ; proc_macro :: TokenStream :: from (input . quote ()) }
};
}
