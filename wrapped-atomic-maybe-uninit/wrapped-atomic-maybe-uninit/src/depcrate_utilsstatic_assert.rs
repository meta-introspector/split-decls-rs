// Generated macro for static_assert (macro)
macro_rules! Depcrate_utilsstatic_assert {
() => {
// Module: crate::utils
// Provides: {"static_assert"}
// Dependencies: {}
# [doc = " Static assertion without depending on const block which requires Rust 1.79."] macro_rules ! static_assert { ($ (const $ consts : ident : $ ty : ty) ,+ => $ ($ tt : tt) *) => { { const_eval ! ($ (const $ consts : $ ty) ,+ => () { assert ! ($ ($ tt) *) }) } } ; ($ ($ ty_params : ident $ (: ? Sized $ (+ $ bounds : path) ?) ?) ,+ => $ ($ tt : tt) *) => { { const_eval ! ($ ($ ty_params $ (: ? Sized $ (+ $ bounds) ?) ?) ,+ => () { assert ! ($ ($ tt) *) }) } } ; ($ ($ ty_params : ident $ (: $ bounds : path) ?) ,+ => $ ($ tt : tt) *) => { { const_eval ! ($ ($ ty_params $ (: $ bounds) ?) ,+ => () { assert ! ($ ($ tt) *) }) } } ; }
};
}
