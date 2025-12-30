// Generated macro for impl_3211 (impl)
macro_rules! Depcrate_pg_connection_cursorimpl_3211 {
() => {
// Module: crate::pg::connection::cursor
// Provides: {"impl_3211"}
// Dependencies: {}
impl Iterator for RowByRowCursor < '_ , '_ > { type Item = crate :: QueryResult < PgRow > ; fn next (& mut self) -> Option < Self :: Item > { if ! self . first_row { let get_next_result = super :: update_transaction_manager_status (self . conn . raw_connection . get_next_result () , self . conn , & crate :: debug_query (& self . query) , false ,) ; match get_next_result { Ok (Some (res)) => { if let Some (old_res) = Rc :: get_mut (& mut self . db_result) { * old_res = res ; } else { self . db_result = Rc :: new (res) ; } } Ok (None) => { return None ; } Err (e) => return Some (Err (e)) , } } if self . db_result . num_rows () > 0 { debug_assert_eq ! (self . db_result . num_rows () , 1) ; self . first_row = false ; Some (Ok (self . db_result . clone () . get_row (0))) } else { None } } }
};
}
