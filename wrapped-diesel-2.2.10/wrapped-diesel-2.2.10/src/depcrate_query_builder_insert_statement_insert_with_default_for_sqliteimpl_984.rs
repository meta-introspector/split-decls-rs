// Generated macro for impl_984 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_984 {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_984"}
// Dependencies: {}
# [allow (unsafe_code)] impl < 'a , T , V , QId , Op , const STATIC_QUERY_ID : bool > DebugQueryHelper < No > for DebugQuery < 'a , InsertStatement < T , BatchInsert < V , T , QId , STATIC_QUERY_ID > , Op > , Sqlite > where T : Copy + QuerySource , Op : Copy , DebugQuery < 'a , InsertStatement < T , SqliteBatchInsertWrapper < V , T , QId , STATIC_QUERY_ID > , Op > , Sqlite , > : Debug + Display , { fn fmt_debug (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let value = unsafe { & * (self as * const DebugQuery < 'a , InsertStatement < T , BatchInsert < V , T , QId , STATIC_QUERY_ID > , Op > , Sqlite , > as * const DebugQuery < 'a , InsertStatement < T , SqliteBatchInsertWrapper < V , T , QId , STATIC_QUERY_ID > , Op > , Sqlite , >) } ; < _ as Debug > :: fmt (value , f) } fn fmt_display (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let value = unsafe { & * (self as * const DebugQuery < 'a , InsertStatement < T , BatchInsert < V , T , QId , STATIC_QUERY_ID > , Op > , Sqlite , > as * const DebugQuery < 'a , InsertStatement < T , SqliteBatchInsertWrapper < V , T , QId , STATIC_QUERY_ID > , Op > , Sqlite , >) } ; < _ as Display > :: fmt (value , f) } }
};
}
