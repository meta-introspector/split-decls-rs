// Generated macro for impl_3190 (impl)
macro_rules! Depcrate_pg_connection_copyimpl_3190 {
() => {
// Module: crate::pg::connection::copy
// Provides: {"impl_3190"}
// Dependencies: {}
impl < 'conn > CopyFromSink < 'conn > { pub (super) fn new (conn : & 'conn mut RawConnection) -> Self { Self { conn } } pub (super) fn finish (self , err : Option < String >) -> QueryResult < () > { self . conn . finish_copy_from (err) } }
};
}
