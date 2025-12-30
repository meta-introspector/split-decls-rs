// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [doc = " Marks async test functions to use the `actix` system entry-point."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[actix::test]"] # [doc = " async fn my_test() {"] # [doc = "     assert!(true);"] # [doc = " }"] # [doc = " ```"] # [proc_macro_attribute] pub fn test (_ : TokenStream , item : TokenStream) -> TokenStream { let mut output : TokenStream = (quote ! { # [:: actix :: __private :: test (system = "::actix::System")] }) . into () ; output . extend (item) ; output }
};
}
