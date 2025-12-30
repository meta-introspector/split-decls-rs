// Generated macro for in_automatically_derived (function)
macro_rules! Depcratein_automatically_derived {
() => {
// Module: crate
// Provides: {"in_automatically_derived"}
// Dependencies: {}
# [doc = " Checks if the given HIR node is inside an `impl` block with the `automatically_derived`"] # [doc = " attribute."] pub fn in_automatically_derived (tcx : TyCtxt < '_ > , id : HirId) -> bool { tcx . hir_parent_owner_iter (id) . filter (| (_ , node) | matches ! (node , OwnerNode :: Item (item) if matches ! (item . kind , ItemKind :: Impl (_)))) . any (| (id , _) | { find_attr ! (tcx . hir_attrs (tcx . local_def_id_to_hir_id (id . def_id)) , AttributeKind :: AutomaticallyDerived (..)) }) }
};
}
