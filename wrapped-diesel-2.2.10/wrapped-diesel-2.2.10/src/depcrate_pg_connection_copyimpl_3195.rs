// Generated macro for impl_3195 (impl)
macro_rules! Depcrate_pg_connection_copyimpl_3195 {
() => {
// Module: crate::pg::connection::copy
// Provides: {"impl_3195"}
// Dependencies: {}
impl Read for CopyToBuffer < '_ > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let data = self . fill_buf () ? ; let len = usize :: min (buf . len () , data . len ()) ; buf [.. len] . copy_from_slice (& data [.. len]) ; self . consume (len) ; Ok (len) } }
};
}
