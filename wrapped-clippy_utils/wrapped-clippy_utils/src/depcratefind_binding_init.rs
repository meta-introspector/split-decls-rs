// Generated macro for find_binding_init (function)
macro_rules! Depcratefind_binding_init {
() => {
// Module: crate
// Provides: {"find_binding_init"}
// Dependencies: {}
# [doc = " Finds the initializer expression for a local binding. Returns `None` if the binding is mutable."] # [doc = ""] # [doc = " By only considering immutable bindings, we guarantee that the returned expression represents the"] # [doc = " value of the binding wherever it is referenced."] # [doc = ""] # [doc = " Example: For `let x = 1`, if the `HirId` of `x` is provided, the `Expr` `1` is returned."] # [doc = " Note: If you have an expression that references a binding `x`, use `path_to_local` to get the"] # [doc = " canonical binding `HirId`."] pub fn find_binding_init < 'tcx > (cx : & LateContext < 'tcx > , hir_id : HirId) -> Option < & 'tcx Expr < 'tcx > > { if let Node :: Pat (pat) = cx . tcx . hir_node (hir_id) && matches ! (pat . kind , PatKind :: Binding (BindingMode :: NONE , ..)) && let Node :: LetStmt (local) = cx . tcx . parent_hir_node (hir_id) { return local . init ; } None }
};
}
