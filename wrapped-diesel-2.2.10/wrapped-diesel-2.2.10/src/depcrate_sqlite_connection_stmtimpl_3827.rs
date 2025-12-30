// Generated macro for impl_3827 (impl)
macro_rules! Depcrate_sqlite_connection_stmtimpl_3827 {
() => {
// Module: crate::sqlite::connection::stmt
// Provides: {"impl_3827"}
// Dependencies: {}
impl Drop for BoundStatement < '_ , '_ > { fn drop (& mut self) { self . statement . reset () ; for (idx , buffer) in std :: mem :: take (& mut self . binds_to_free) { unsafe { self . statement . bind (SqliteType :: Text , InternalSqliteBindValue :: Null , idx) . expect ("Binding a null value should never fail. \
                             If you ever see this error message please open \
                             an issue at diesels issue tracker containing \
                             code how to trigger this message." ,) ; } if let Some (buffer) = buffer { unsafe { std :: mem :: drop (Box :: from_raw (buffer . as_ptr ())) ; } } } if let Some (query) = self . query { let query = unsafe { Box :: from_raw (query . as_ptr ()) } ; if ! self . has_error { self . instrumentation . on_connection_event (crate :: connection :: InstrumentationEvent :: FinishQuery { query : & crate :: debug_query (& query) , error : None , } ,) ; } std :: mem :: drop (query) ; self . query = None ; } } }
};
}
