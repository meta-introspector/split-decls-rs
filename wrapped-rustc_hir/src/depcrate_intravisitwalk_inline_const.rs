// Generated macro for walk_inline_const (function)
macro_rules! Depcrate_intravisitwalk_inline_const {
() => {
// Module: crate::intravisit
// Provides: {"walk_inline_const"}
// Dependencies: {}
pub fn walk_inline_const < 'v , V : Visitor < 'v > > (visitor : & mut V , constant : & 'v ConstBlock ,) -> V :: Result { let ConstBlock { hir_id , def_id : _ , body } = constant ; try_visit ! (visitor . visit_id (* hir_id)) ; visitor . visit_nested_body (* body) }
};
}
