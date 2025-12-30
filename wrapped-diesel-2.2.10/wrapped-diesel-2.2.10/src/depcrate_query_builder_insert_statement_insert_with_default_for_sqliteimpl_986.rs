// Generated macro for impl_986 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_986 {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_986"}
// Dependencies: {}
impl < T , V , QId , Op , O , const STATIC_QUERY_ID : bool > Debug for DebugQuery < '_ , InsertStatement < T , BatchInsert < Vec < ValuesClause < V , T > > , T , QId , STATIC_QUERY_ID > , Op > , Sqlite , > where T : QuerySource , V : ContainsDefaultableValue < Out = O > , Self : DebugQueryHelper < O > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . fmt_debug (f) } }
};
}
