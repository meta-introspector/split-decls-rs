// Generated macro for SqliteRow (struct)
macro_rules! Depcrate_sqlite_connection_rowSqliteRow {
() => {
// Module: crate::sqlite::connection::row
// Provides: {"SqliteRow"}
// Dependencies: {}
# [allow (missing_debug_implementations)] pub struct SqliteRow < 'stmt , 'query > { pub (super) inner : Rc < RefCell < PrivateSqliteRow < 'stmt , 'query > > > , pub (super) field_count : usize , }
};
}
