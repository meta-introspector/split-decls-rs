// Generated macro for AsArrayExpression (trait)
macro_rules! Depcrate_pg_expression_array_comparisonAsArrayExpression {
() => {
// Module: crate::pg::expression::array_comparison
// Provides: {"AsArrayExpression"}
// Dependencies: {}
pub trait AsArrayExpression < ST : 'static > { type Expression : Expression < SqlType = Array < ST > > ; # [allow (clippy :: wrong_self_convention)] fn as_expression (self) -> Self :: Expression ; }
};
}
