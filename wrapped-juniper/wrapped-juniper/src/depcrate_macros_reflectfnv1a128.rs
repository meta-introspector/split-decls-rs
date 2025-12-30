// Generated macro for fnv1a128 (function)
macro_rules! Depcrate_macros_reflectfnv1a128 {
() => {
// Module: crate::macros::reflect
// Provides: {"fnv1a128"}
// Dependencies: {}
# [doc = " Non-cryptographic hash with good dispersion to use as a [`str`](prim@str) in"] # [doc = " `const` generics. See [spec] for more info."] # [doc = ""] # [doc = " [spec]: https://datatracker.ietf.org/doc/html/draft-eastlake-fnv-17.html"] # [must_use] pub const fn fnv1a128 (str : Name) -> u128 { const FNV_OFFSET_BASIS : u128 = 0x6c62272e07bb014262b821756295c58d ; const FNV_PRIME : u128 = 0x0000000001000000000000000000013b ; let bytes = str . as_bytes () ; let mut hash = FNV_OFFSET_BASIS ; let mut i = 0 ; while i < bytes . len () { hash ^= bytes [i] as u128 ; hash = hash . wrapping_mul (FNV_PRIME) ; i += 1 ; } hash }
};
}
