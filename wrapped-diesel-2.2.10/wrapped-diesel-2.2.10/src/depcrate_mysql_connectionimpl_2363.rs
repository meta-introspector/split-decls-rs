// Generated macro for impl_2363 (impl)
macro_rules! Depcrate_mysql_connectionimpl_2363 {
() => {
// Module: crate::mysql::connection
// Provides: {"impl_2363"}
// Dependencies: {}
impl LoadConnection < DefaultLoadingMode > for MysqlConnection { type Cursor < 'conn , 'query > = self :: stmt :: iterator :: StatementIterator < 'conn > ; type Row < 'conn , 'query > = self :: stmt :: iterator :: MysqlRow ; fn load < 'conn , 'query , T > (& 'conn mut self , source : T ,) -> QueryResult < Self :: Cursor < 'conn , 'query > > where T : Query + QueryFragment < Self :: Backend > + QueryId + 'query , Self :: Backend : QueryMetadata < T :: SqlType > , { update_transaction_manager_status (prepared_query (& source , & mut self . statement_cache , & mut self . raw_connection , & mut self . instrumentation ,) . and_then (| stmt | { let mut metadata = Vec :: new () ; Mysql :: row_metadata (& mut () , & mut metadata) ; StatementIterator :: from_stmt (stmt , & metadata) }) , & mut self . transaction_state , & mut self . instrumentation , & crate :: debug_query (& source) ,) } }
};
}
