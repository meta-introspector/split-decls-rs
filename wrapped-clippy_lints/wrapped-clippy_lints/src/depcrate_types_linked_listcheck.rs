// Generated macro for check (function)
macro_rules! Depcrate_types_linked_listcheck {
() => {
// Module: crate::types::linked_list
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , hir_ty : & hir :: Ty < '_ > , def_id : DefId) -> bool { if cx . tcx . is_diagnostic_item (sym :: LinkedList , def_id) { span_lint_and_help (cx , LINKEDLIST , hir_ty . span , "you seem to be using a `LinkedList`! Perhaps you meant some other data structure?" , None , "a `VecDeque` might work" ,) ; true } else { false } }
};
}
