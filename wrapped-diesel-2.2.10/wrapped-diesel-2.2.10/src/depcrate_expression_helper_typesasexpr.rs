// Generated macro for AsExpr (type)
macro_rules! Depcrate_expression_helper_typesAsExpr {
() => {
// Module: crate::expression::helper_types
// Provides: {"AsExpr"}
// Dependencies: {}
# [doc = " The type of `Item` when converted to an expression with the same type as `TargetExpr`"] pub type AsExpr < Item , TargetExpr > = AsExprOf < Item , SqlTypeOf < TargetExpr > > ;
};
}
