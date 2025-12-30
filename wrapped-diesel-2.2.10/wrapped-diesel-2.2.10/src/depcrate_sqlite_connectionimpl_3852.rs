// Generated macro for impl_3852 (impl)
macro_rules! Depcrate_sqlite_connectionimpl_3852 {
() => {
// Module: crate::sqlite::connection
// Provides: {"impl_3852"}
// Dependencies: {}
impl SimpleConnection for SqliteConnection { fn batch_execute (& mut self , query : & str) -> QueryResult < () > { self . instrumentation . on_connection_event (InstrumentationEvent :: StartQuery { query : & StrQueryHelper :: new (query) , }) ; let resp = self . raw_connection . exec (query) ; self . instrumentation . on_connection_event (InstrumentationEvent :: FinishQuery { query : & StrQueryHelper :: new (query) , error : resp . as_ref () . err () , }) ; resp } }
};
}
