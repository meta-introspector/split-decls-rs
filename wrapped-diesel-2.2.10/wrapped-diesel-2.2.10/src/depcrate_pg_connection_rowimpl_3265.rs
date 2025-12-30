// Generated macro for impl_3265 (impl)
macro_rules! Depcrate_pg_connection_rowimpl_3265 {
() => {
// Module: crate::pg::connection::row
// Provides: {"impl_3265"}
// Dependencies: {}
impl RowIndex < usize > for PgRow { fn idx (& self , idx : usize) -> Option < usize > { if idx < self . field_count () { Some (idx) } else { None } } }
};
}
