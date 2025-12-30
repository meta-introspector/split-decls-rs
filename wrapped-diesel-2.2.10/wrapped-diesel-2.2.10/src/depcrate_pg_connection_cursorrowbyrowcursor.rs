// Generated macro for RowByRowCursor (struct)
macro_rules! Depcrate_pg_connection_cursorRowByRowCursor {
() => {
// Module: crate::pg::connection::cursor
// Provides: {"RowByRowCursor"}
// Dependencies: {}
# [doc = " The type returned by various [`Connection`] methods."] # [doc = " Acts as an iterator over `T`."] # [allow (missing_debug_implementations)] pub struct RowByRowCursor < 'conn , 'query > { first_row : bool , db_result : Rc < PgResult > , conn : & 'conn mut super :: ConnectionAndTransactionManager , query : Box < dyn QueryFragment < Pg > + 'query > , }
};
}
