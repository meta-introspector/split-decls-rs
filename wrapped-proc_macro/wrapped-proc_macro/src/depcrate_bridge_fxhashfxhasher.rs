// Generated macro for FxHasher (struct)
macro_rules! Depcrate_bridge_fxhashFxHasher {
() => {
// Module: crate::bridge::fxhash
// Provides: {"FxHasher"}
// Dependencies: {}
# [doc = " A speedy hash algorithm for use within rustc. The hashmap in alloc by"] # [doc = " default uses SipHash which isn't quite as speedy as we want. In the compiler"] # [doc = " we're not really worried about DOS attempts, so we use a fast"] # [doc = " non-cryptographic hash."] # [doc = ""] # [doc = " This is the same as the algorithm used by Firefox -- which is a homespun"] # [doc = " one not based on any widely-known algorithm -- though modified to produce"] # [doc = " 64-bit hash values instead of 32-bit hash values. It consistently"] # [doc = " out-performs an FNV-based hash within rustc itself -- the collision rate is"] # [doc = " similar or slightly worse than FNV, but the speed of the hash function"] # [doc = " itself is much higher because it works on up to 8 bytes at a time."] # [derive (Default)] pub (super) struct FxHasher { hash : usize , }
};
}
