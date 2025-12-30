// Generated macro for postgres (module)
macro_rules! Depcrate_query_builder_combination_clausepostgres {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"postgres"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] mod postgres { use super :: * ; use crate :: pg :: Pg ; impl < T : QueryFragment < Pg > > QueryFragment < Pg > for ParenthesisWrapper < T > { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . push_sql ("(") ; self . 0 . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } } impl SupportsCombinationClause < Union , Distinct > for Pg { } impl SupportsCombinationClause < Union , All > for Pg { } impl SupportsCombinationClause < Intersect , Distinct > for Pg { } impl SupportsCombinationClause < Intersect , All > for Pg { } impl SupportsCombinationClause < Except , Distinct > for Pg { } impl SupportsCombinationClause < Except , All > for Pg { } }
};
}
