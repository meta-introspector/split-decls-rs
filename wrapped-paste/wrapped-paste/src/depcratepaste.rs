// Generated macro for paste (function)
macro_rules! Depcratepaste {
() => {
// Module: crate
// Provides: {"paste"}
// Dependencies: {}
# [proc_macro] pub fn paste (input : TokenStream) -> TokenStream { let mut contains_paste = false ; let flatten_single_interpolation = true ; match expand (input . clone () , & mut contains_paste , flatten_single_interpolation ,) { Ok (expanded) => { if contains_paste { expanded } else { input } } Err (err) => err . to_compile_error () , } }
};
}
