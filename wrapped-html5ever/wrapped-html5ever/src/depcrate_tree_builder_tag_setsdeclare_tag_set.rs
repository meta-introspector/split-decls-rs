// Generated macro for declare_tag_set (macro)
macro_rules! Depcrate_tree_builder_tag_setsdeclare_tag_set {
() => {
// Module: crate::tree_builder::tag_sets
// Provides: {"declare_tag_set"}
// Dependencies: {}
macro_rules ! declare_tag_set ((pub $ name : ident = $ ($ toks : tt) +) => (pub fn $ name (p : :: string_cache :: QualName) -> bool { declare_tag_set_body ! (p = $ ($ toks) +) }) ; ($ name : ident = $ ($ toks : tt) +) => (fn $ name (p : :: string_cache :: QualName) -> bool { declare_tag_set_body ! (p = $ ($ toks) +) }) ;) ;
};
}
