// Generated macro for StableHasher (struct)
macro_rules! Depcrate_stable_hasherStableHasher {
() => {
// Module: crate::stable_hasher
// Provides: {"StableHasher"}
// Dependencies: {}
# [doc = " A Stable Hasher adapted for cross-platform independent hash."] # [doc = ""] # [doc = " When hashing something that ends up affecting properties like symbol names,"] # [doc = " we want these symbol names to be calculated independently of other factors"] # [doc = " like what architecture you're compiling *from*."] # [doc = ""] # [doc = " To that end we always convert integers to little-endian format before"] # [doc = " hashing and the architecture dependent `isize` and `usize` types are"] # [doc = " extended to 64 bits if needed."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rustc_stable_hash::hashers::{StableSipHasher128, SipHasher128Hash};"] # [doc = " use rustc_stable_hash::{StableHasher, FromStableHash};"] # [doc = " use std::hash::Hasher;"] # [doc = ""] # [doc = " struct Hash128([u64; 2]);"] # [doc = " impl FromStableHash for Hash128 {"] # [doc = "     type Hash = SipHasher128Hash;"] # [doc = ""] # [doc = "     fn from(SipHasher128Hash(hash): SipHasher128Hash) -> Hash128 {"] # [doc = "         Hash128(hash)"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let mut hasher = StableSipHasher128::new();"] # [doc = " hasher.write_usize(0xFA);"] # [doc = ""] # [doc = " let hash: Hash128 = hasher.finish();"] # [doc = " ```"] # [must_use] # [derive (Clone)] pub struct StableHasher < H : ExtendedHasher > { state : H , }
};
}
