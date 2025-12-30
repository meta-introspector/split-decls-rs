// Generated macro for blk (function)
macro_rules! Depcrate_imp_sha1blk {
() => {
// Module: crate::imp::sha1
// Provides: {"blk"}
// Dependencies: {}
const fn blk (block : & [u32] , i : usize) -> u32 { let value = block [(i + 13) & 15] ^ block [(i + 8) & 15] ^ block [(i + 2) & 15] ^ block [i] ; rol (value , 1) }
};
}
