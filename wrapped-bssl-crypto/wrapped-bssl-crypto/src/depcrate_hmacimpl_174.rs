// Generated macro for impl_174 (impl)
macro_rules! Depcrate_hmacimpl_174 {
() => {
// Module: crate::hmac
// Provides: {"impl_174"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: io :: Write for HmacSha512 { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . update (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
