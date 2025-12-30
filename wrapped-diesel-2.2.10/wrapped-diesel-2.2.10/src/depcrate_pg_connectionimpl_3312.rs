// Generated macro for impl_3312 (impl)
macro_rules! Depcrate_pg_connectionimpl_3312 {
() => {
// Module: crate::pg::connection
// Provides: {"impl_3312"}
// Dependencies: {}
impl SimpleConnection for PgConnection { # [allow (unsafe_code)] fn batch_execute (& mut self , query : & str) -> QueryResult < () > { self . connection_and_transaction_manager . instrumentation . on_connection_event (InstrumentationEvent :: StartQuery { query : & StrQueryHelper :: new (query) , }) ; let c_query = CString :: new (query) ? ; let inner_result = unsafe { self . connection_and_transaction_manager . raw_connection . exec (c_query . as_ptr ()) } ; update_transaction_manager_status (inner_result . and_then (| raw_result | { PgResult :: new (raw_result , & self . connection_and_transaction_manager . raw_connection ,) }) , & mut self . connection_and_transaction_manager , & StrQueryHelper :: new (query) , true ,) ? ; Ok (()) } }
};
}
