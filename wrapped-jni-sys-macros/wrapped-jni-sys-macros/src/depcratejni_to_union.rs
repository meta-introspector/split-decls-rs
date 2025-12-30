// Generated macro for jni_to_union (function)
macro_rules! Depcratejni_to_union {
() => {
// Module: crate
// Provides: {"jni_to_union"}
// Dependencies: {}
# [proc_macro_attribute] pub fn jni_to_union (_attr : TokenStream , item : TokenStream) -> TokenStream { let input = parse_macro_input ! (item as DeriveInput) ; match jni_to_union_impl (input) { Ok (tokens) => tokens , Err (err) => err . into_compile_error () . into () , } }
};
}
