// Generated macro for impl_3206 (impl)
macro_rules! Depcrate_pg_connection_cursorimpl_3206 {
() => {
// Module: crate::pg::connection::cursor
// Provides: {"impl_3206"}
// Dependencies: {}
impl Cursor { pub (super) fn new (result : PgResult , conn : & mut RawConnection) -> crate :: QueryResult < Cursor > { let next_res = conn . get_next_result () ? ; debug_assert ! (next_res . is_none ()) ; Ok (Self { current_row : 0 , db_result : Rc :: new (result) , }) } }
};
}
