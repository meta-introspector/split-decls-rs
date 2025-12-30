// Generated macro for property_test (function)
macro_rules! Depcrate_property_testproperty_test {
() => {
// Module: crate::property_test
// Provides: {"property_test"}
// Dependencies: {}
pub fn property_test (attr : TokenStream , item : TokenStream) -> TokenStream { let mut item_fn = parse ! (item) ; let options = parse ! (attr) ; if let Err (compile_error) = validate (& mut item_fn) { return compile_error ; } codegen :: generate (item_fn , options) }
};
}
