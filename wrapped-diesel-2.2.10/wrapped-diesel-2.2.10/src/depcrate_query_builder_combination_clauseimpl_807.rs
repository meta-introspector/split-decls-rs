// Generated macro for impl_807 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_807 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_807"}
// Dependencies: {}
impl < Combinator , Rule , Source , Rhs > CombinationClause < Combinator , Rule , Source , Rhs > { # [doc = " Create a new combination"] pub (crate) fn new (combinator : Combinator , duplicate_rule : Rule , source : Source , rhs : Rhs ,) -> Self { CombinationClause { combinator , duplicate_rule , source : ParenthesisWrapper (source) , rhs : ParenthesisWrapper (rhs) , } } }
};
}
