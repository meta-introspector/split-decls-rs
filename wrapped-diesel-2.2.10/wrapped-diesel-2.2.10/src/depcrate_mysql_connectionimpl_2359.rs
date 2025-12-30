// Generated macro for impl_2359 (impl)
macro_rules! Depcrate_mysql_connectionimpl_2359 {
() => {
// Module: crate::mysql::connection
// Provides: {"impl_2359"}
// Dependencies: {}
impl SimpleConnection for MysqlConnection { fn batch_execute (& mut self , query : & str) -> QueryResult < () > { self . instrumentation . on_connection_event (InstrumentationEvent :: StartQuery { query : & StrQueryHelper :: new (query) , }) ; let r = self . raw_connection . enable_multi_statements (| | self . raw_connection . execute (query)) ; self . instrumentation . on_connection_event (InstrumentationEvent :: FinishQuery { query : & StrQueryHelper :: new (query) , error : r . as_ref () . err () , }) ; r } }
};
}
