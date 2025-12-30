// Generated macro for mimes (macro)
macro_rules! Depcrate_constantsmimes {
() => {
// Module: crate::constants
// Provides: {"mimes"}
// Dependencies: {}
macro_rules ! mimes { ($ ($ id : ident , $ ($ piece : expr) ,+;) +) => (# [allow (non_camel_case_types)] enum __Atoms { __Dynamic , $ ($ id ,) + } $ (mime_constant ! { $ id , $ ($ piece) ,+ }) + # [test] fn test_mimes_macro_consts () { $ (mime_constant_test ! { $ id , $ ($ piece) ,* }) + }) }
};
}
