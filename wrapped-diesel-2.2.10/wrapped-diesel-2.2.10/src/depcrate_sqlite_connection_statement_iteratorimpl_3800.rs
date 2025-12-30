// Generated macro for impl_3800 (impl)
macro_rules! Depcrate_sqlite_connection_statement_iteratorimpl_3800 {
() => {
// Module: crate::sqlite::connection::statement_iterator
// Provides: {"impl_3800"}
// Dependencies: {}
impl < 'stmt , 'query > StatementIterator < 'stmt , 'query > { pub fn new (stmt : StatementUse < 'stmt , 'query >) -> StatementIterator < 'stmt , 'query > { Self { inner : PrivateStatementIterator :: NotStarted (Some (stmt)) , column_names : None , field_count : 0 , } } }
};
}
