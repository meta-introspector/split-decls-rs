// Generated macro for impl_3268 (impl)
macro_rules! Depcrate_pg_connection_rowimpl_3268 {
() => {
// Module: crate::pg::connection::row
// Provides: {"impl_3268"}
// Dependencies: {}
impl < 'a > Field < 'a , Pg > for PgField < 'a > { fn field_name (& self) -> Option < & str > { self . db_result . column_name (self . col_idx) } fn value (& self) -> Option < < Pg as Backend > :: RawValue < '_ > > { let raw = self . db_result . get (self . row_idx , self . col_idx) ? ; Some (PgValue :: new_internal (raw , self)) } }
};
}
