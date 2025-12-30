// Generated macro for impl_3266 (impl)
macro_rules! Depcrate_pg_connection_rowimpl_3266 {
() => {
// Module: crate::pg::connection::row
// Provides: {"impl_3266"}
// Dependencies: {}
impl < 'a > RowIndex < & 'a str > for PgRow { fn idx (& self , field_name : & 'a str) -> Option < usize > { (0 .. self . field_count ()) . find (| idx | self . db_result . column_name (* idx) == Some (field_name)) } }
};
}
