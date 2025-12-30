// Generated macro for PrivateStatementIterator (enum)
macro_rules! Depcrate_sqlite_connection_statement_iteratorPrivateStatementIterator {
() => {
// Module: crate::sqlite::connection::statement_iterator
// Provides: {"PrivateStatementIterator"}
// Dependencies: {}
enum PrivateStatementIterator < 'stmt , 'query > { NotStarted (Option < StatementUse < 'stmt , 'query > >) , Started (Rc < RefCell < PrivateSqliteRow < 'stmt , 'query > > >) , }
};
}
