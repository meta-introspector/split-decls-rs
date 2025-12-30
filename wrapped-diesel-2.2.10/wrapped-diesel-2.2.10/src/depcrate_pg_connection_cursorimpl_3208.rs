// Generated macro for impl_3208 (impl)
macro_rules! Depcrate_pg_connection_cursorimpl_3208 {
() => {
// Module: crate::pg::connection::cursor
// Provides: {"impl_3208"}
// Dependencies: {}
impl Iterator for Cursor { type Item = crate :: QueryResult < PgRow > ; fn next (& mut self) -> Option < Self :: Item > { if self . current_row < self . db_result . num_rows () { let row = self . db_result . clone () . get_row (self . current_row) ; self . current_row += 1 ; Some (Ok (row)) } else { None } } fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . current_row = (self . current_row + n) . min (self . db_result . num_rows ()) ; self . next () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } fn count (self) -> usize where Self : Sized , { self . len () } }
};
}
