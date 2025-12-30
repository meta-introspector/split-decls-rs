// Generated macro for impl_7 (impl)
macro_rules! Depcrate_decodeimpl_7 {
() => {
// Module: crate::decode
// Provides: {"impl_7"}
// Dependencies: {}
impl Header { # [doc = " Parse blobby header"] pub const fn parse (data : & mut & [u8]) -> Result < Self , Error > { match (read_vlq (data) , read_vlq (data)) { (Ok (items_len) , Ok (dedup_len)) => Ok (Header { items_len , dedup_len , }) , (Err (err) , _) | (Ok (_) , Err (err)) => Err (err) , } } }
};
}
