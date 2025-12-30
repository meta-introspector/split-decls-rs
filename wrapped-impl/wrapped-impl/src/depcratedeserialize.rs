// Generated macro for deserialize (function)
macro_rules! Depcratedeserialize {
() => {
// Module: crate
// Provides: {"deserialize"}
// Dependencies: {}
# [proc_macro_attribute] pub fn deserialize (args : TokenStream , input : TokenStream) -> TokenStream { let ser = false ; let de = true ; expand (args , input , Mode { ser , de }) }
};
}
