// Generated macro for prepared_query (function)
macro_rules! Depcrate_mysql_connectionprepared_query {
() => {
// Module: crate::mysql::connection
// Provides: {"prepared_query"}
// Dependencies: {}
fn prepared_query < 'a , T : QueryFragment < Mysql > + QueryId > (source : & '_ T , statement_cache : & 'a mut StatementCache < Mysql , Statement > , raw_connection : & 'a mut RawConnection , instrumentation : & mut dyn Instrumentation ,) -> QueryResult < MaybeCached < 'a , Statement > > { instrumentation . on_connection_event (InstrumentationEvent :: StartQuery { query : & crate :: debug_query (source) , }) ; let mut stmt = statement_cache . cached_statement (source , & Mysql , & [] , | sql , _ | raw_connection . prepare (sql) , instrumentation ,) ? ; let mut bind_collector = RawBytesBindCollector :: new () ; source . collect_binds (& mut bind_collector , & mut () , & Mysql) ? ; let binds = bind_collector . metadata . into_iter () . zip (bind_collector . binds) ; stmt . bind (binds) ? ; Ok (stmt) }
};
}
