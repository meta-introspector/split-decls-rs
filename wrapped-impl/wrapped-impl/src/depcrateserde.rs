// Generated macro for serde (function)
macro_rules! Depcrateserde {
() => {
// Module: crate
// Provides: {"serde"}
// Dependencies: {}
# [proc_macro_attribute] pub fn serde (args : TokenStream , input : TokenStream) -> TokenStream { let ser = true ; let de = true ; expand (args , input , Mode { ser , de }) }
};
}
