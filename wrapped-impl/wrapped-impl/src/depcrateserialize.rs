// Generated macro for serialize (function)
macro_rules! Depcrateserialize {
() => {
// Module: crate
// Provides: {"serialize"}
// Dependencies: {}
# [proc_macro_attribute] pub fn serialize (args : TokenStream , input : TokenStream) -> TokenStream { let ser = true ; let de = false ; expand (args , input , Mode { ser , de }) }
};
}
