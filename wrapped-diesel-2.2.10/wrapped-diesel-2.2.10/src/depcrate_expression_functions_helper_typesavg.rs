// Generated macro for avg (type)
macro_rules! Depcrate_expression_functions_helper_typesavg {
() => {
// Module: crate::expression::functions::helper_types
// Provides: {"avg"}
// Dependencies: {}
# [doc = " The return type of [`avg(expr)`](crate::dsl::avg())"] pub type avg < Expr > = super :: aggregate_folding :: avg < SqlTypeOf < Expr > , Expr > ;
};
}
