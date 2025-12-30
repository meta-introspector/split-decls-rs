// Generated macro for Murmur3Hasher (struct)
macro_rules! Depcrate_murmur3Murmur3Hasher {
() => {
// Module: crate::murmur3
// Provides: {"Murmur3Hasher"}
// Dependencies: {}
# [doc = " 32-bit `MurmurHash3` hasher"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::hash::Hasher as _;"] # [doc = " use hash32::{Hasher as _, Murmur3Hasher};"] # [doc = ""] # [doc = " let mut hasher: Murmur3Hasher = Default::default();"] # [doc = " hasher.write(b\"Hello, World!\");"] # [doc = ""] # [doc = " println!(\"Hash is {:x}!\", hasher.finish32());"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct Murmur3Hasher { buf : Buffer , index : Index , processed : u32 , state : State , }
};
}
