// Generated macro for impl_3562 (impl)
macro_rules! Depcrate_pg_query_builder_tablesampleimpl_3562 {
() => {
// Module: crate::pg::query_builder::tablesample
// Provides: {"impl_3562"}
// Dependencies: {}
impl < S , TSM > QueryFragment < Pg > for Tablesample < S , TSM > where S : QueryFragment < Pg > , TSM : TablesampleMethod , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { self . source . walk_ast (out . reborrow ()) ? ; out . push_sql (" TABLESAMPLE ") ; out . push_sql (TSM :: method_name_sql ()) ; out . push_sql ("(") ; out . push_bind_param :: < SmallInt , _ > (& self . portion) ? ; out . push_sql (")") ; if let Some (f) = & self . seed { out . push_sql (" REPEATABLE(") ; out . push_bind_param :: < Double , _ > (f) ? ; out . push_sql (")") ; } Ok (()) } }
};
}
