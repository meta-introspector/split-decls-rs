// Generated macro for impl_2418 (impl)
macro_rules! Depcrate_io_read_to_stringimpl_2418 {
() => {
// Module: crate::io::read_to_string
// Provides: {"impl_2418"}
// Dependencies: {}
impl < 'a , R : AsyncRead + ? Sized + Unpin > ReadToString < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut String) -> Self { let start_len = buf . len () ; Self { reader , bytes : mem :: take (buf) . into_bytes () , buf , start_len } } }
};
}
