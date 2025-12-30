// Generated macro for update_transaction_manager_status (function)
macro_rules! Depcrate_mysql_connectionupdate_transaction_manager_status {
() => {
// Module: crate::mysql::connection
// Provides: {"update_transaction_manager_status"}
// Dependencies: {}
# [inline (always)] fn update_transaction_manager_status < T > (query_result : QueryResult < T > , transaction_manager : & mut AnsiTransactionManager , instrumentation : & mut Option < Box < dyn Instrumentation > > , query : & dyn DebugQuery ,) -> QueryResult < T > { if let Err (Error :: DatabaseError (DatabaseErrorKind :: SerializationFailure , _)) = query_result { transaction_manager . status . set_requires_rollback_maybe_up_to_top_level (true) } instrumentation . on_connection_event (InstrumentationEvent :: FinishQuery { query , error : query_result . as_ref () . err () , }) ; query_result }
};
}
