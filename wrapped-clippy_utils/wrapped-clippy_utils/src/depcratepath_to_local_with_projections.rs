// Generated macro for path_to_local_with_projections (function)
macro_rules! Depcratepath_to_local_with_projections {
() => {
// Module: crate
// Provides: {"path_to_local_with_projections"}
// Dependencies: {}
# [doc = " If the expression is a path to a local (with optional projections),"] # [doc = " returns the canonical `HirId` of the local."] # [doc = ""] # [doc = " For example, `x.field[0].field2` would return the `HirId` of `x`."] pub fn path_to_local_with_projections (expr : & Expr < '_ >) -> Option < HirId > { match expr . kind { ExprKind :: Field (recv , _) | ExprKind :: Index (recv , _ , _) => path_to_local_with_projections (recv) , ExprKind :: Path (QPath :: Resolved (_ , Path { res : Res :: Local (local) , .. } ,)) => Some (* local) , _ => None , } }
};
}
