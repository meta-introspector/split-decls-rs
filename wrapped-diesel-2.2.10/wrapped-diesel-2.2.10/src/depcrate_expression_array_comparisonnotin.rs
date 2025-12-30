// Generated macro for NotIn (struct)
macro_rules! Depcrate_expression_array_comparisonNotIn {
() => {
// Module: crate::expression::array_comparison
// Provides: {"NotIn"}
// Dependencies: {}
# [doc = " Query dsl node that represents a `left NOT IN (values)`"] # [doc = " expression"] # [doc = ""] # [doc = " Third party backend can customize the [`QueryFragment`]"] # [doc = " implementation of this query dsl node via"] # [doc = " [`SqlDialect::ArrayComparison`]. A customized implementation"] # [doc = " is expected to provide the same semantics as an ANSI SQL"] # [doc = " `NOT IN` expression.0"] # [doc = ""] # [doc = " The postgres backend provided a specialized implementation"] # [doc = " by using `left = ALL(values)` as optimized variant instead."] # [derive (Debug , Copy , Clone , QueryId , ValidGrouping)] # [non_exhaustive] pub struct NotIn < T , U > { # [doc = " The expression on the left side of the `NOT IN` keyword"] pub left : T , # [doc = " The values clause of the `NOT IN` expression"] pub values : U , }
};
}
