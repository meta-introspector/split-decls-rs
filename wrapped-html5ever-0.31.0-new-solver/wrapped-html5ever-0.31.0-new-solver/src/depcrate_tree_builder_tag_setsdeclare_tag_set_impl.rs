// Generated macro for declare_tag_set_impl (macro)
macro_rules! Depcrate_tree_builder_tag_setsdeclare_tag_set_impl {
() => {
// Module: crate::tree_builder::tag_sets
// Provides: {"declare_tag_set_impl"}
// Dependencies: {}
macro_rules ! declare_tag_set_impl (($ param : ident , $ b : ident , $ supr : ident , $ ($ tag : tt) +) => (match $ param { $ (expanded_name ! (html $ tag) => $ b ,) + p => $ supr (p) , })) ;
};
}
