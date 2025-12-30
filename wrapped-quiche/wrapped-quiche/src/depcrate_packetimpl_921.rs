// Generated macro for impl_921 (impl)
macro_rules! Depcrate_packetimpl_921 {
() => {
// Module: crate::packet
// Provides: {"impl_921"}
// Dependencies: {}
impl CryptoContext { pub fn new () -> CryptoContext { let crypto_stream = stream :: Stream :: new (0 , u64 :: MAX , u64 :: MAX , true , true , stream :: MAX_STREAM_WINDOW ,) ; CryptoContext { key_update : None , crypto_open : None , crypto_seal : None , crypto_0rtt_open : None , crypto_stream , } } pub fn clear (& mut self) { self . crypto_open = None ; self . crypto_seal = None ; self . crypto_stream = < stream :: Stream > :: new (0 , u64 :: MAX , u64 :: MAX , true , true , stream :: MAX_STREAM_WINDOW ,) ; } pub fn data_available (& self) -> bool { self . crypto_stream . is_flushable () } pub fn crypto_overhead (& self) -> Option < usize > { Some (self . crypto_seal . as_ref () ? . alg () . tag_len ()) } pub fn has_keys (& self) -> bool { self . crypto_open . is_some () && self . crypto_seal . is_some () } }
};
}
