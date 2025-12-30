// Generated macro for derive_ex (function)
macro_rules! Depcratederive_ex {
() => {
// Module: crate
// Provides: {"derive_ex"}
// Dependencies: {}
# [doc = include_str ! ("../../doc/derive_ex.md")] # [proc_macro_attribute] pub fn derive_ex (attr : proc_macro :: TokenStream , item : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { let mut item : TokenStream = item . into () ; match build (attr . into () , item . clone ()) { Ok (s) => s , Err (e) => { item . extend (e . to_compile_error ()) ; item } } . into () }
};
}
