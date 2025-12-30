// Generated macro for Expressions (enum)
macro_rules! Depcrate_coercionExpressions {
() => {
// Module: crate::coercion
// Provides: {"Expressions"}
// Dependencies: {}
enum Expressions < 'tcx , 'exprs , E : AsCoercionSite > { Dynamic (Vec < & 'tcx hir :: Expr < 'tcx > >) , UpFront (& 'exprs [E]) , }
};
}
