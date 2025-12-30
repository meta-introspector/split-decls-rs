// Generated macro for names (macro)
macro_rules! Depcrate_constantsnames {
() => {
// Module: crate::constants
// Provides: {"names"}
// Dependencies: {}
macro_rules ! names { ($ ($ id : ident , $ e : expr ;) *) => (pub mod names { $ (names ! { @ DOC concat ! ("The string literal `\"" , $ e , "\"`.") , $ id , $ e }) * # [test] fn test_names_macro_consts () { $ (assert_eq ! ($ id . to_ascii_lowercase () , $ id) ;) * } }) ; (@ DOC $ doc : expr , $ id : ident , $ e : expr) => (# [doc = $ doc] pub const $ id : &'static str = $ e ;) }
};
}
