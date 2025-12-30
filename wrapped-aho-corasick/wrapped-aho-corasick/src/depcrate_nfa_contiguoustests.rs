// Generated macro for tests (module)
macro_rules! Depcrate_nfa_contiguoustests {
() => {
// Module: crate::nfa::contiguous
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [cfg (target_endian = "little")] # [test] fn swar () { use super :: * ; fn has_zero_byte (x : u32) -> u32 { const LO_U32 : u32 = 0x01010101 ; const HI_U32 : u32 = 0x80808080 ; x . wrapping_sub (LO_U32) & ! x & HI_U32 } fn broadcast (b : u8) -> u32 { (u32 :: from (b)) * (u32 :: MAX / 255) } fn index_of (x : u32) -> usize { let o = (((x - 1) & 0x01010101) . wrapping_mul (0x01010101) >> 24) - 1 ; o . as_usize () } let bytes : [u8 ; 4] = [b'1' , b'A' , b'a' , b'z'] ; let chunk = u32 :: from_ne_bytes (bytes) ; let needle = broadcast (b'1') ; assert_eq ! (0 , index_of (has_zero_byte (needle ^ chunk))) ; let needle = broadcast (b'A') ; assert_eq ! (1 , index_of (has_zero_byte (needle ^ chunk))) ; let needle = broadcast (b'a') ; assert_eq ! (2 , index_of (has_zero_byte (needle ^ chunk))) ; let needle = broadcast (b'z') ; assert_eq ! (3 , index_of (has_zero_byte (needle ^ chunk))) ; } }
};
}
