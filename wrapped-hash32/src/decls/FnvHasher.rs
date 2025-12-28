macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! FnvHasher {
    () => {
        deps!();
        # [doc = " 32-bit Fowler-Noll-Vo hasher"] # [doc = ""] # [doc = " Specifically this implements the [FNV-1a hash]."] # [doc = ""] # [doc = " [FNV-1a hash]: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1a_hash"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::hash::Hasher as _;"] # [doc = " use hash32::{FnvHasher, Hasher as _};"] # [doc = ""] # [doc = " let mut hasher: FnvHasher = Default::default();"] # [doc = " hasher.write(b\"Hello, World!\");"] # [doc = ""] # [doc = " println!(\"Hash is {:x}!\", hasher.finish32());"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct FnvHasher { state : u32 , }
    };
}

FnvHasher!();