// Generated macro for impl_325 (impl)
macro_rules! Depcrate_stream_logimpl_325 {
() => {
// Module: crate::stream::log
// Provides: {"impl_325"}
// Dependencies: {}
impl < S : Write , W : Write > Write for LogStream < S , W > { fn write (& mut self , buf : & [u8]) -> Result < usize > { let n = self . stream . write (buf) ? ; self . log_write (& buf [.. n]) ; Ok (n) } fn flush (& mut self) -> Result < () > { self . stream . flush () } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> Result < usize > { let n = self . stream . write_vectored (bufs) ? ; let mut rest = n ; let mut bytes = Vec :: new () ; for buf in bufs { let written = std :: cmp :: min (buf . len () , rest) ; rest -= written ; bytes . extend (& buf . as_ref () [.. written]) ; if rest == 0 { break ; } } self . log_write (& bytes) ; Ok (n) } }
};
}
