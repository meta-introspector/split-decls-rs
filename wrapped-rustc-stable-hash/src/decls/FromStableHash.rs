macro_rules! deps {
    () => {
        StableHasher!();
    };
}

macro_rules! FromStableHash {
    () => {
        deps!();
        # [doc = " Trait for processing the result of the stable hashing operation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rustc_stable_hash::{StableHasher, FromStableHash};"] # [doc = ""] # [doc = " struct Hash128(u128);"] # [doc = ""] # [doc = " impl FromStableHash for Hash128 {"] # [doc = "     type Hash = [u64; 2];"] # [doc = ""] # [doc = "     fn from(hash: [u64; 2]) -> Hash128 {"] # [doc = "         let upper: u128 = hash[0] as u128;"] # [doc = "         let lower: u128 = hash[1] as u128;"] # [doc = ""] # [doc = "         Hash128((upper << 64) | lower)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub trait FromStableHash : Sized { type Hash ; # [doc = " Convert the finalized state of a [`StableHasher`] and construct"] # [doc = " an [`Self`] containing the processed hash."] fn from (hash : Self :: Hash) -> Self ; }
    };
}

FromStableHash!()