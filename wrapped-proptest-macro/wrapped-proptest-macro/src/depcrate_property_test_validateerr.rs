// Generated macro for err (function)
macro_rules! Depcrate_property_test_validateerr {
() => {
// Module: crate::property_test::validate
// Provides: {"err"}
// Dependencies: {}
# [doc = " Helper function to generate `compile_error!()` outputs"] fn err (span : impl Spanned , s : & str) -> Result < () , TokenStream > { Err (quote_spanned ! { span . span () => compile_error ! (# s) }) }
};
}
