// Generated macro for get_parent_as_impl (function)
macro_rules! Depcrateget_parent_as_impl {
() => {
// Module: crate
// Provides: {"get_parent_as_impl"}
// Dependencies: {}
# [doc = " Gets the parent node if it's an impl block."] pub fn get_parent_as_impl (tcx : TyCtxt < '_ > , id : HirId) -> Option < & Impl < '_ > > { match tcx . hir_parent_iter (id) . next () { Some ((_ , Node :: Item (Item { kind : ItemKind :: Impl (imp) , .. }) ,)) => Some (imp) , _ => None , } }
};
}
