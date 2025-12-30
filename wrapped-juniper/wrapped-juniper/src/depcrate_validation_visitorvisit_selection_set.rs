// Generated macro for visit_selection_set (function)
macro_rules! Depcrate_validation_visitorvisit_selection_set {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_selection_set"}
// Dependencies: {}
fn visit_selection_set < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , selection_set : & 'a [Selection < S >] ,) where S : ScalarValue , V : Visitor < 'a , S > , { ctx . with_pushed_parent_type (| ctx | { v . enter_selection_set (ctx , selection_set) ; for selection in selection_set . iter () { visit_selection (v , ctx , selection) ; } v . exit_selection_set (ctx , selection_set) ; }) ; }
};
}
