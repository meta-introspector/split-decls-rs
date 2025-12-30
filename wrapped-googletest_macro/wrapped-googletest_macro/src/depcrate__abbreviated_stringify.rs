// Generated macro for __abbreviated_stringify (function)
macro_rules! Depcrate__abbreviated_stringify {
() => {
// Module: crate
// Provides: {"__abbreviated_stringify"}
// Dependencies: {}
# [doc = " Stringifies its argument (like `stringify!()` from the standard library) but"] # [doc = " limits the output to a provided maximum length."] # [doc = ""] # [doc = " The input is a tuple of `target` and `max_length` seprated by a comma."] # [doc = " The `max_length` is the maximum number of characters to include in the"] # [doc = " abbreviated string. For example:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[rstest]"] # [doc = " #[gtest]"] # [doc = " fn test_abbreviated_string() -> Result<()> {"] # [doc = "   verifiy_eq!(__abbreviated_stringify!(|x| x + 1, 6), \"|x|...\")?;"] # [doc = "   Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc (hidden)] # [proc_macro] pub fn __abbreviated_stringify (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = input . to_string () ; let abbreviated = abbreviated_string (& input) . unwrap () ; quote ! { # abbreviated } . into () }
};
}
