// Generated macro for attr_panic (function)
macro_rules! Depcrateattr_panic {
() => {
// Module: crate
// Provides: {"attr_panic"}
// Dependencies: {}
# [proc_macro_attribute] pub fn attr_panic (args : TokenStream , item : TokenStream) -> TokenStream { panic ! ("#[attr_panic {}] {}" , args , item) ; }
};
}
