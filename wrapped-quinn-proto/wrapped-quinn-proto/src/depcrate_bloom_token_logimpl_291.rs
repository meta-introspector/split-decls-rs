// Generated macro for impl_291 (impl)
macro_rules! Depcrate_bloom_token_logimpl_291 {
() => {
// Module: crate::bloom_token_log
// Provides: {"impl_291"}
// Dependencies: {}
impl Filter { fn check_and_insert (& mut self , fingerprint : u64 , config : & FilterConfig ,) -> Result < () , TokenReuseError > { match self { Self :: Set (hset) => { if ! hset . insert (fingerprint) { return Err (TokenReuseError) ; } if hset . capacity () * size_of :: < u64 > () <= config . filter_max_bytes { return Ok (()) ; } let mut bloom = BloomFilter :: with_num_bits ((config . filter_max_bytes * 8) . max (1)) . hasher (FxBuildHasher) . hashes (config . k_num) ; for item in & * hset { bloom . insert (item) ; } * self = Self :: Bloom (bloom) ; } Self :: Bloom (bloom) => { if bloom . insert (& fingerprint) { return Err (TokenReuseError) ; } } } Ok (()) } }
};
}
