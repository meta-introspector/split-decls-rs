// Generated macro for sqlite (module)
macro_rules! Depcrate_query_builder_combination_clausesqlite {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"sqlite"}
// Dependencies: {}
# [cfg (feature = "sqlite")] mod sqlite { use super :: * ; use crate :: sqlite :: Sqlite ; impl < T : QueryFragment < Sqlite > > QueryFragment < Sqlite > for ParenthesisWrapper < T > { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Sqlite >) -> QueryResult < () > { out . push_sql ("SELECT * FROM (") ; self . 0 . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } } impl SupportsCombinationClause < Union , Distinct > for Sqlite { } impl SupportsCombinationClause < Union , All > for Sqlite { } impl SupportsCombinationClause < Intersect , Distinct > for Sqlite { } impl SupportsCombinationClause < Except , Distinct > for Sqlite { } }
};
}
