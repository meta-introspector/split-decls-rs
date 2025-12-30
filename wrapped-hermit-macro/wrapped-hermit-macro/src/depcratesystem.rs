// Generated macro for system (function)
macro_rules! Depcratesystem {
() => {
// Module: crate
// Provides: {"system"}
// Dependencies: {}
# [proc_macro_attribute] pub fn system (attr : TokenStream , item : TokenStream) -> TokenStream { match system :: system_attribute (parse_macro_input ! (attr) , parse_macro_input ! (item)) { Ok (item) => item . into_token_stream () . into () , Err (e) => e . to_compile_error () . into () , } }
};
}
