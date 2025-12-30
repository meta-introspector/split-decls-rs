// Generated macro for impl_3191 (impl)
macro_rules! Depcrate_pg_connection_copyimpl_3191 {
() => {
// Module: crate::pg::connection::copy
// Provides: {"impl_3191"}
// Dependencies: {}
impl Write for CopyFromSink < '_ > { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . conn . put_copy_data (buf) . map_err (| e | std :: io :: Error :: new (std :: io :: ErrorKind :: Other , e)) ? ; Ok (buf . len ()) } fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
