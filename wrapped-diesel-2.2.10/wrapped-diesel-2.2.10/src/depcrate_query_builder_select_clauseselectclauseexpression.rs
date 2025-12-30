// Generated macro for SelectClauseExpression (trait)
macro_rules! Depcrate_query_builder_select_clauseSelectClauseExpression {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"SelectClauseExpression"}
// Dependencies: {}
# [doc = " Specialised variant of `Expression` for select clause types"] # [doc = ""] # [doc = " The difference to the normal `Expression` trait is the query source (`QS`)"] # [doc = " generic type parameter. This allows to access the query source in generic code."] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] pub trait SelectClauseExpression < QS > { # [doc = " The expression represented by the given select clause"] type Selection ; # [doc = " SQL type of the select clause"] type SelectClauseSqlType ; }
};
}
