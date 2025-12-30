// Generated macro for impl_1002 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_1002 {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_1002"}
// Dependencies: {}
impl < V , T , QId , C , Op , O , const STATIC_QUERY_ID : bool > ExecuteDsl < C , Sqlite > for InsertStatement < T , BatchInsert < Vec < ValuesClause < V , T > > , T , QId , STATIC_QUERY_ID > , Op > where T : QuerySource , C : Connection < Backend = Sqlite > , V : ContainsDefaultableValue < Out = O > , O : Default , (O , Self) : ExecuteDsl < C , Sqlite > , { fn execute (query : Self , conn : & mut C) -> QueryResult < usize > { < (O , Self) as ExecuteDsl < C , Sqlite > > :: execute ((O :: default () , query) , conn) } }
};
}
