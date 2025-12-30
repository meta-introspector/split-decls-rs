// Generated macro for is_def_id_trait_method (function)
macro_rules! Depcrateis_def_id_trait_method {
() => {
// Module: crate
// Provides: {"is_def_id_trait_method"}
// Dependencies: {}
# [doc = " Checks if the `def_id` belongs to a function that is part of a trait impl."] pub fn is_def_id_trait_method (cx : & LateContext < '_ > , def_id : LocalDefId) -> bool { if let Node :: Item (item) = cx . tcx . parent_hir_node (cx . tcx . local_def_id_to_hir_id (def_id)) && let ItemKind :: Impl (imp) = item . kind { imp . of_trait . is_some () } else { false } }
};
}
