// Generated macro for In (struct)
macro_rules! Depcrate_expression_array_comparisonIn {
() => {
// Module: crate::expression::array_comparison
// Provides: {"In"}
// Dependencies: {}
# [doc = " Query dsl node that represents a `left IN (values)`"] # [doc = " expression"] # [doc = ""] # [doc = " Third party backend can customize the [`QueryFragment`]"] # [doc = " implementation of this query dsl node via"] # [doc = " [`SqlDialect::ArrayComparison`]. A customized implementation"] # [doc = " is expected to provide the same semantics as an ANSI SQL"] # [doc = " `IN` expression."] # [doc = ""] # [doc = " The postgres backend provided a specialized implementation"] # [doc = " by using `left = ANY(values)` as optimized variant instead."] # [derive (Debug , Copy , Clone , QueryId , ValidGrouping)] # [non_exhaustive] pub struct In < T , U > { # [doc = " The expression on the left side of the `IN` keyword"] pub left : T , # [doc = " The values clause of the `IN` expression"] pub values : U , }
};
}
