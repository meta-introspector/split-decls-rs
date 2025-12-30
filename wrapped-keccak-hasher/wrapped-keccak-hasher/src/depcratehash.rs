// Generated macro for hash (function)
macro_rules! Depcratehash {
() => {
// Module: crate
// Provides: {"hash"}
// Dependencies: {}
# [doc = " Return a Keccak256 hash for the given data."] # [cfg_attr (any (target_os = "solana" , target_arch = "bpf") , inline (always))] pub fn hash (val : & [u8]) -> Hash { hashv (& [val]) }
};
}
