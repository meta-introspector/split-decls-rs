// Generated macro for Hasher (trait)
macro_rules! DepcrateHasher {
() => {
// Module: crate
// Provides: {"Hasher"}
// Dependencies: {}
# [doc = " An extension of [`core::hash::Hasher`] for 32-bit hashers."] # [doc = ""] # [doc = " For hashers that implement this trait, the [`core::hash::Hasher::finish`] method should return a"] # [doc = " zero-extended version of the result from [`Hasher::finish32`]."] # [doc = ""] # [doc = " # Contract"] # [doc = ""] # [doc = " Implementers of this trait must **not** perform any 64-bit (or 128-bit) operation while computing"] # [doc = " the hash."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::hash::Hasher as _;"] # [doc = " use hash32::{FnvHasher, Hasher as _};"] # [doc = ""] # [doc = " let mut hasher: FnvHasher = Default::default();"] # [doc = ""] # [doc = " hasher.write_u32(1989);"] # [doc = " hasher.write_u8(11);"] # [doc = " hasher.write_u8(9);"] # [doc = " hasher.write(b\"Huh?\");"] # [doc = ""] # [doc = " println!(\"Hash is {:x}!\", hasher.finish32());"] # [doc = " ```"] pub trait Hasher : core :: hash :: Hasher { # [doc = " The equivalent of [`core::hash::Hasher::finish`] for 32-bit hashers."] # [doc = ""] # [doc = " This returns the hash directly; [`core::hash::Hasher::finish`] zero-extends the `finish32`"] # [doc = " result to 64-bits for compatibility."] fn finish32 (& self) -> u32 ; }
};
}
