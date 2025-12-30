// Generated macro for visit_selection_set (function)
macro_rules! Depcrate_validation_visitorvisit_selection_set {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_selection_set"}
// Dependencies: {}
fn visit_selection_set < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , selection_set : & 'a Positioned < SelectionSet > ,) { if ! selection_set . node . items . is_empty () { v . enter_selection_set (ctx , selection_set) ; for selection in & selection_set . node . items { visit_selection (v , ctx , selection) ; } v . exit_selection_set (ctx , selection_set) ; } }
};
}
