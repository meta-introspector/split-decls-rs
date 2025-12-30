// Generated macro for impl_30 (impl)
macro_rules! Depcrate_bundle_write_typesimpl_30 {
() => {
// Module: crate::bundle::write::types
// Provides: {"impl_30"}
// Dependencies: {}
impl < R > io :: Read for PassThrough < R > where R : io :: Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let bytes_read = self . reader . read (buf) ? ; if let Some (writer) = self . writer . as_mut () { use std :: io :: Write ; writer . lock () . write_all (& buf [.. bytes_read]) ? ; } Ok (bytes_read) } }
};
}
