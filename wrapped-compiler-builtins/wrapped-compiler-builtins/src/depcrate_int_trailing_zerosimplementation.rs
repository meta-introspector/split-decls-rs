// Generated macro for implementation (module)
macro_rules! Depcrate_int_trailing_zerosimplementation {
() => {
// Module: crate::int::trailing_zeros
// Provides: {"implementation"}
// Dependencies: {}
mod implementation { use crate :: int :: { CastFrom , Int } ; # [doc = " Returns number of trailing binary zeros in `x`."] # [allow (dead_code)] pub fn trailing_zeros < I : Int > (x : I) -> usize where u32 : CastFrom < I > , u16 : CastFrom < I > , u8 : CastFrom < I > , { let mut x = x ; let mut r : u32 = 0 ; let mut t : u32 ; const { assert ! (I :: BITS <= 64) } ; if I :: BITS >= 64 { r += ((u32 :: cast_from_lossy (x) == 0) as u32) << 5 ; x >>= r ; } if I :: BITS >= 32 { t = ((u16 :: cast_from_lossy (x) == 0) as u32) << 4 ; r += t ; x >>= t ; } const { assert ! (I :: BITS >= 16) } ; t = ((u8 :: cast_from_lossy (x) == 0) as u32) << 3 ; x >>= t ; r += t ; let mut x : u8 = x . cast_lossy () ; t = (((x & 0x0F) == 0) as u32) << 2 ; x >>= t ; r += t ; t = (((x & 0x3) == 0) as u32) << 1 ; x >>= t ; r += t ; x &= 3 ; r as usize + ((2 - (x >> 1) as usize) & (((x & 1) == 0) as usize) . wrapping_neg ()) } }
};
}
