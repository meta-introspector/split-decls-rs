// Generated macro for declare_tag_set_body (macro)
macro_rules! Depcrate_tree_builder_tag_setsdeclare_tag_set_body {
() => {
// Module: crate::tree_builder::tag_sets
// Provides: {"declare_tag_set_body"}
// Dependencies: {}
macro_rules ! declare_tag_set_body (($ param : ident = [$ supr : ident] - $ ($ tag : tt) +) => (declare_tag_set_impl ! ($ param , false , $ supr , $ ($ tag) +)) ; ($ param : ident = [$ supr : ident] + $ ($ tag : tt) +) => (declare_tag_set_impl ! ($ param , true , $ supr , $ ($ tag) +)) ; ($ param : ident = $ ($ tag : tt) +) => (declare_tag_set_impl ! ($ param , true , empty_set , $ ($ tag) +)) ;) ;
};
}
