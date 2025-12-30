// Generated macro for conditional_wrapper (function)
macro_rules! Depcrateconditional_wrapper {
() => {
// Module: crate
// Provides: {"conditional_wrapper"}
// Dependencies: {}
# [doc = " Conditionally replace module based on feature flag"] # [doc = " "] # [doc = " Usage:"] # [doc = " ```rust"] # [doc = " conditional_wrapper!("] # [doc = "     feature = \"use_wrapped_alloc\","] # [doc = "     original = std::alloc,"] # [doc = "     wrapper = \"generated/alloc_wrapper\""] # [doc = " );"] # [doc = " ```"] # [proc_macro] pub fn conditional_wrapper (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as ConditionalWrapperInput) ; let feature = & input . feature ; let original = & input . original ; let wrapper = & input . wrapper ; quote ! { # [cfg (feature = # feature)] pub use # wrapper as # original ; # [cfg (not (feature = # feature))] pub use # original ; } . into () }
};
}
