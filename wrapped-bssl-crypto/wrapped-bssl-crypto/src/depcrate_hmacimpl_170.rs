// Generated macro for impl_170 (impl)
macro_rules! Depcrate_hmacimpl_170 {
() => {
// Module: crate::hmac
// Provides: {"impl_170"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: io :: Write for HmacSha256 { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . update (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
