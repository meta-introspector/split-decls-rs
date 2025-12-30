// Generated macro for for_each_value_source (function)
macro_rules! Depcrate_visitorsfor_each_value_source {
() => {
// Module: crate::visitors
// Provides: {"for_each_value_source"}
// Dependencies: {}
# [doc = " Runs the given function for each sub-expression producing the final value consumed by the parent"] # [doc = " of the give expression."] # [doc = ""] # [doc = " e.g. for the following expression"] # [doc = " ```rust,ignore"] # [doc = " if foo {"] # [doc = "     f(0)"] # [doc = " } else {"] # [doc = "     1 + 1"] # [doc = " }"] # [doc = " ```"] # [doc = " this will pass both `f(0)` and `1+1` to the given function."] pub fn for_each_value_source < 'tcx , B > (e : & 'tcx Expr < 'tcx > , f : & mut impl FnMut (& 'tcx Expr < 'tcx >) -> ControlFlow < B > ,) -> ControlFlow < B > { match e . kind { ExprKind :: Block (Block { expr : Some (e) , .. } , _) => for_each_value_source (e , f) , ExprKind :: Match (_ , arms , _) => { for arm in arms { for_each_value_source (arm . body , f) ? ; } ControlFlow :: Continue (()) } , ExprKind :: If (_ , if_expr , Some (else_expr)) => { for_each_value_source (if_expr , f) ? ; for_each_value_source (else_expr , f) } , ExprKind :: DropTemps (e) => for_each_value_source (e , f) , _ => f (e) , } }
};
}
