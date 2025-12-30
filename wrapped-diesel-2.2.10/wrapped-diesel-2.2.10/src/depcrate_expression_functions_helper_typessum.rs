// Generated macro for sum (type)
macro_rules! Depcrate_expression_functions_helper_typessum {
() => {
// Module: crate::expression::functions::helper_types
// Provides: {"sum"}
// Dependencies: {}
# [doc = " The return type of [`sum(expr)`](crate::dsl::sum())"] pub type sum < Expr > = super :: aggregate_folding :: sum < SqlTypeOf < Expr > , Expr > ;
};
}
