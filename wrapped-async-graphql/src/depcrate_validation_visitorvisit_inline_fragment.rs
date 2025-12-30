// Generated macro for visit_inline_fragment (function)
macro_rules! Depcrate_validation_visitorvisit_inline_fragment {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_inline_fragment"}
// Dependencies: {}
fn visit_inline_fragment < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , inline_fragment : & 'a Positioned < InlineFragment > ,) { v . enter_inline_fragment (ctx , inline_fragment) ; visit_directives (v , ctx , & inline_fragment . node . directives) ; visit_selection_set (v , ctx , & inline_fragment . node . selection_set) ; v . exit_inline_fragment (ctx , inline_fragment) ; }
};
}
