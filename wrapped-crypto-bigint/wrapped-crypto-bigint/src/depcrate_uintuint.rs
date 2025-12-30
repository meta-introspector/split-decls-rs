// Generated macro for Uint (struct)
macro_rules! Depcrate_uintUint {
() => {
// Module: crate::uint
// Provides: {"Uint"}
// Dependencies: {}
# [doc = " Stack-allocated big unsigned integer."] # [doc = ""] # [doc = " Generic over the given number of `LIMBS`"] # [doc = ""] # [doc = " # Encoding support"] # [doc = " This type supports many different types of encodings, either via the"] # [doc = " [`Encoding`][`crate::Encoding`] trait or various `const fn` decoding and"] # [doc = " encoding functions that can be used with [`Uint`] constants."] # [doc = ""] # [doc = " Optional crate features for encoding (off-by-default):"] # [doc = " - `hybrid-array`: enables [`ArrayEncoding`][`crate::ArrayEncoding`] trait which can be used to"] # [doc = "   [`Uint`] as `Array<u8, N>` and a [`ArrayDecoding`][`crate::ArrayDecoding`] trait which"] # [doc = "   can be used to `Array<u8, N>` as [`Uint`]."] # [doc = " - `rlp`: support for [Recursive Length Prefix (RLP)][RLP] encoding."] # [doc = ""] # [doc = " [RLP]: https://eth.wiki/fundamentals/rlp"] # [allow (clippy :: derived_hash_with_manual_eq)] # [derive (Copy , Clone , Hash)] pub struct Uint < const LIMBS : usize > { # [doc = " Inner limb array. Stored from least significant to most significant."] pub (crate) limbs : [Limb ; LIMBS] , }
};
}
