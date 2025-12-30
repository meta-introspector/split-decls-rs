// Generated macro for impl_3212 (impl)
macro_rules! Depcrate_pg_connection_cursorimpl_3212 {
() => {
// Module: crate::pg::connection::cursor
// Provides: {"impl_3212"}
// Dependencies: {}
impl Drop for RowByRowCursor < '_ , '_ > { fn drop (& mut self) { loop { let res = super :: update_transaction_manager_status (self . conn . raw_connection . get_next_result () , self . conn , & crate :: debug_query (& self . query) , false ,) ; if matches ! (res , Err (_) | Ok (None)) { if res . is_ok () { self . conn . instrumentation . on_connection_event (crate :: connection :: InstrumentationEvent :: FinishQuery { query : & crate :: debug_query (& self . query) , error : None , } ,) ; } break ; } } } }
};
}
