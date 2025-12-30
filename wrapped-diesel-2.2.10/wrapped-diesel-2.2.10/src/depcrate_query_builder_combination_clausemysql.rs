// Generated macro for mysql (module)
macro_rules! Depcrate_query_builder_combination_clausemysql {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"mysql"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] mod mysql { use super :: * ; use crate :: mysql :: Mysql ; impl < T : QueryFragment < Mysql > > QueryFragment < Mysql > for ParenthesisWrapper < T > { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { out . push_sql ("(") ; self . 0 . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } } impl SupportsCombinationClause < Union , Distinct > for Mysql { } impl SupportsCombinationClause < Union , All > for Mysql { } }
};
}
