// Generated macro for unpack_match (function)
macro_rules! Depcrate_unused_io_amountunpack_match {
() => {
// Module: crate::unused_io_amount
// Provides: {"unpack_match"}
// Dependencies: {}
fn unpack_match < 'a > (mut expr : & 'a hir :: Expr < 'a >) -> & 'a hir :: Expr < 'a > { while let ExprKind :: Match (res , _ , _) = expr . kind { expr = res ; } expr }
};
}
