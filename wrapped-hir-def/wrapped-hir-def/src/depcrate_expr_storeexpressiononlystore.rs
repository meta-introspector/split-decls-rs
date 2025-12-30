// Generated macro for ExpressionOnlyStore (struct)
macro_rules! Depcrate_expr_storeExpressionOnlyStore {
() => {
// Module: crate::expr_store
// Provides: {"ExpressionOnlyStore"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq)] struct ExpressionOnlyStore { exprs : Arena < Expr > , pats : Arena < Pat > , bindings : Arena < Binding > , labels : Arena < Label > , # [doc = " Id of the closure/coroutine that owns the corresponding binding. If a binding is owned by the"] # [doc = " top level expression, it will not be listed in here."] binding_owners : FxHashMap < BindingId , ExprId > , # [doc = " Block expressions in this store that may contain inner items."] block_scopes : Box < [BlockId] > , # [doc = " A map from an variable usages to their hygiene ID."] # [doc = ""] # [doc = " Expressions (and destructuing patterns) that can be recorded here are single segment path, although not all single segments path refer"] # [doc = " to variables and have hygiene (some refer to items, we don't know at this stage)."] ident_hygiene : FxHashMap < ExprOrPatId , HygieneId > , }
};
}
