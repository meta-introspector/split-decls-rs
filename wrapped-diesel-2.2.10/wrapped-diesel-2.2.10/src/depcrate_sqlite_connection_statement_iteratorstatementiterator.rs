// Generated macro for StatementIterator (struct)
macro_rules! Depcrate_sqlite_connection_statement_iteratorStatementIterator {
() => {
// Module: crate::sqlite::connection::statement_iterator
// Provides: {"StatementIterator"}
// Dependencies: {}
# [allow (missing_debug_implementations)] pub struct StatementIterator < 'stmt , 'query > { inner : PrivateStatementIterator < 'stmt , 'query > , column_names : Option < Rc < [Option < String >] > > , field_count : usize , }
};
}
