// Generated macro for impl_1132 (impl)
macro_rules! Depcrate_query_builder_select_clauseimpl_1132 {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"impl_1132"}
// Dependencies: {}
impl < QS , DB > QueryFragment < DB > for DefaultSelectClause < QS > where DB : Backend , QS : AsQuerySource , < QS :: QuerySource as QuerySource > :: DefaultSelection : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . default_selection . walk_ast (pass) } }
};
}
