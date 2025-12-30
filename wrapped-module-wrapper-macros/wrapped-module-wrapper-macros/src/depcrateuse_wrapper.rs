// Generated macro for use_wrapper (function)
macro_rules! Depcrateuse_wrapper {
() => {
// Module: crate
// Provides: {"use_wrapper"}
// Dependencies: {}
# [doc = " Replace a module/function with a generated wrapper"] # [doc = " "] # [doc = " Usage:"] # [doc = " ```rust"] # [doc = " use_wrapper!(original_module::function_name => \"path/to/generated/wrapper\");"] # [doc = " ```"] # [proc_macro] pub fn use_wrapper (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as UseWrapperInput) ; let original_path = & input . original_path ; let wrapper_path = & input . wrapper_path ; quote ! { pub use # wrapper_path as # original_path ; } . into () }
};
}
