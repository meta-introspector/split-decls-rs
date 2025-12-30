// Generated macro for impl_3210 (impl)
macro_rules! Depcrate_pg_connection_cursorimpl_3210 {
() => {
// Module: crate::pg::connection::cursor
// Provides: {"impl_3210"}
// Dependencies: {}
impl < 'conn , 'query > RowByRowCursor < 'conn , 'query > { pub (super) fn new (db_result : PgResult , conn : & 'conn mut super :: ConnectionAndTransactionManager , query : Box < dyn QueryFragment < Pg > + 'query > ,) -> Self { RowByRowCursor { first_row : true , db_result : Rc :: new (db_result) , conn , query , } } }
};
}
