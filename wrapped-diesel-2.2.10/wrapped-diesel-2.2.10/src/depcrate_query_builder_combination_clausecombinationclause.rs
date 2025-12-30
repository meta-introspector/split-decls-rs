// Generated macro for CombinationClause (struct)
macro_rules! Depcrate_query_builder_combination_clauseCombinationClause {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"CombinationClause"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , QueryId)] # [must_use = "Queries are only executed when calling `load`, `get_result` or similar."] # [doc = " Combine queries using a combinator like `UNION`, `INTERSECT` or `EXPECT`"] # [doc = " with or without `ALL` rule for duplicates"] pub struct CombinationClause < Combinator , Rule , Source , Rhs > { combinator : Combinator , duplicate_rule : Rule , source : ParenthesisWrapper < Source > , rhs : ParenthesisWrapper < Rhs > , }
};
}
