// Generated macro for AsExpressionList (trait)
macro_rules! Depcrate_expressionAsExpressionList {
() => {
// Module: crate::expression
// Provides: {"AsExpressionList"}
// Dependencies: {}
# [doc = " Converts a tuple of values into a tuple of Diesel expressions."] # [doc = ""] # [doc = " This trait is similar to [`AsExpression`], but it operates on tuples."] # [doc = " The expressions must all be of the same SQL type."] # [doc = ""] pub trait AsExpressionList < ST > { # [doc = " The final output expression"] type Expression ; # [doc = " Perform the conversion"] # [allow (clippy :: wrong_self_convention)] fn as_expression_list (self) -> Self :: Expression ; }
};
}
