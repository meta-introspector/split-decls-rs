// Generated macro for ExpressionStore (struct)
macro_rules! Depcrate_expr_storeExpressionStore {
() => {
// Module: crate::expr_store
// Provides: {"ExpressionStore"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq)] pub struct ExpressionStore { expr_only : Option < Box < ExpressionOnlyStore > > , pub types : Arena < TypeRef > , pub lifetimes : Arena < LifetimeRef > , }
};
}
