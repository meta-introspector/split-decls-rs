// Generated macro for Hasher (struct)
macro_rules! DepcrateHasher {
() => {
// Module: crate
// Provides: {"Hasher"}
// Dependencies: {}
# [derive (Clone , Default)] # [cfg (all (feature = "blake3" , not (any (target_os = "solana" , target_arch = "bpf"))))] pub struct Hasher { hasher : blake3 :: Hasher , }
};
}
