// Generated macro for PrivateSqliteRow (enum)
macro_rules! Depcrate_sqlite_connection_rowPrivateSqliteRow {
() => {
// Module: crate::sqlite::connection::row
// Provides: {"PrivateSqliteRow"}
// Dependencies: {}
pub (super) enum PrivateSqliteRow < 'stmt , 'query > { Direct (StatementUse < 'stmt , 'query >) , Duplicated { values : Vec < Option < OwnedSqliteValue > > , column_names : Rc < [Option < String >] > , } , }
};
}
