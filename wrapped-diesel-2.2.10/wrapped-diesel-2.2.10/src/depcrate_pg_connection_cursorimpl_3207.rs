// Generated macro for impl_3207 (impl)
macro_rules! Depcrate_pg_connection_cursorimpl_3207 {
() => {
// Module: crate::pg::connection::cursor
// Provides: {"impl_3207"}
// Dependencies: {}
impl ExactSizeIterator for Cursor { fn len (& self) -> usize { self . db_result . num_rows () - self . current_row } }
};
}
