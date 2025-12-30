// Generated macro for any_parent_has_attr (function)
macro_rules! Depcrateany_parent_has_attr {
() => {
// Module: crate
// Provides: {"any_parent_has_attr"}
// Dependencies: {}
pub fn any_parent_has_attr (tcx : TyCtxt < '_ > , node : HirId , symbol : Symbol) -> bool { let mut prev_enclosing_node = None ; let mut enclosing_node = node ; while Some (enclosing_node) != prev_enclosing_node { if has_attr (tcx . hir_attrs (enclosing_node) , symbol) { return true ; } prev_enclosing_node = Some (enclosing_node) ; enclosing_node = tcx . hir_get_parent_item (enclosing_node) . into () ; } false }
};
}
