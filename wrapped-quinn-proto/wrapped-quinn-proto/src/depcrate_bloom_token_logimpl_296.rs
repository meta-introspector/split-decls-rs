// Generated macro for impl_296 (impl)
macro_rules! Depcrate_bloom_token_logimpl_296 {
() => {
// Module: crate::bloom_token_log
// Provides: {"impl_296"}
// Dependencies: {}
impl Hasher for IdentityHasher { fn write (& mut self , bytes : & [u8]) { # [cfg (debug_assertions)] { assert ! (! self . wrote_8_byte_slice) ; assert_eq ! (bytes . len () , 8) ; self . wrote_8_byte_slice = true ; } self . data . copy_from_slice (bytes) ; } fn finish (& self) -> u64 { # [cfg (debug_assertions)] assert ! (self . wrote_8_byte_slice) ; u64 :: from_ne_bytes (self . data) } }
};
}
