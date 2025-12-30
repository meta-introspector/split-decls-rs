// Generated macro for U32 (type)
macro_rules! Depcrate_integrations_ruintU32 {
() => {
// Module: crate::integrations::ruint
// Provides: {"U32"}
// Dependencies: {}
# [doc = " Unsigned integer type representing the ring of numbers modulo 2<sup>32</sup>."] # [doc = ""] # [doc = " Always serializes as `String` in decimal notation. But may be deserialized both from `Int` and"] # [doc = " `String` values with standard Rust syntax for decimal, hexadecimal, binary and octal notation"] # [doc = " using prefixes `0x`, `0b` and `0o`."] # [doc = ""] # [doc = " See also [`ruint`] crate for details."] # [doc = ""] # [doc = " [`ruint`]: https://docs.rs/ruint"] # [graphql_scalar] # [graphql (with = uint_scalar , specified_by_url = "https://docs.rs/ruint")] pub type U32 = ruint :: aliases :: U32 ;
};
}
