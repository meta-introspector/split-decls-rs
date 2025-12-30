// Generated macro for AsInExpression (trait)
macro_rules! Depcrate_expression_array_comparisonAsInExpression {
() => {
// Module: crate::expression::array_comparison
// Provides: {"AsInExpression"}
// Dependencies: {}
# [doc = " This trait describes how a type is transformed to the"] # [doc = " `IN (values)` value expression"] # [doc = ""] # [doc = " Diesel provided several implementations here:"] # [doc = ""] # [doc = "  - An implementation for any [`Iterator`] over values"] # [doc = "    that implement [`AsExpression<ST>`] for the corresponding"] # [doc = "    sql type ST. The corresponding values clause will contain"] # [doc = "    bind statements for each individual value."] # [doc = "  - An implementation for select statements, that returns"] # [doc = "    a single field. The corresponding values clause will contain"] # [doc = "    the sub query."] # [doc = ""] # [doc = "  This trait is exposed for custom third party backends so"] # [doc = "  that they can restrict the [`QueryFragment`] implementations"] # [doc = "  for [`In`] and [`NotIn`]."] pub trait AsInExpression < T : SqlType + TypedExpressionType > { # [doc = " Type of the expression returned by [AsInExpression::as_in_expression]"] type InExpression : MaybeEmpty + Expression < SqlType = T > ; # [doc = " Construct the diesel query dsl representation of"] # [doc = " the `IN (values)` clause for the given type"] # [allow (clippy :: wrong_self_convention)] fn as_in_expression (self) -> Self :: InExpression ; }
};
}
