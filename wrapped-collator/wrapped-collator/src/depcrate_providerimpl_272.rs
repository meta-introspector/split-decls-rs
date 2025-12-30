// Generated macro for impl_272 (impl)
macro_rules! Depcrate_providerimpl_272 {
() => {
// Module: crate::provider
// Provides: {"impl_272"}
// Dependencies: {}
impl CollationSpecialPrimariesValidated < '_ > { # [expect (clippy :: unwrap_used)] pub (crate) fn last_primary_for_group (& self , max_variable : MaxVariable) -> u32 { (u32 :: from (self . last_primaries . get (max_variable as usize) . unwrap ()) << 16) - 1 } # [allow (dead_code)] pub (crate) fn is_compressible (& self , b : u8) -> bool { # [expect (clippy :: indexing_slicing)] let field = u16 :: from_unaligned (self . compressible_bytes [usize :: from (b >> 4)]) ; let mask = 1 << (b & 0b1111) ; (field & mask) != 0 } }
};
}
