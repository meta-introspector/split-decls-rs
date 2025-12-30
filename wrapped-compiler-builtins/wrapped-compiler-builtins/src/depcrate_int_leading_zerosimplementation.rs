// Generated macro for implementation (module)
macro_rules! Depcrate_int_leading_zerosimplementation {
() => {
// Module: crate::int::leading_zeros
// Provides: {"implementation"}
// Dependencies: {}
mod implementation { use crate :: int :: { CastFrom , Int } ; # [doc = " Returns the number of leading binary zeros in `x`."] # [allow (dead_code)] pub fn leading_zeros_default < I : Int > (x : I) -> usize where usize : CastFrom < I > , { let mut x = x ; let mut z = I :: BITS as usize ; let mut t : I ; const { assert ! (I :: BITS <= 64) } ; if I :: BITS >= 64 { t = x >> 32 ; if t != I :: ZERO { z -= 32 ; x = t ; } } if I :: BITS >= 32 { t = x >> 16 ; if t != I :: ZERO { z -= 16 ; x = t ; } } const { assert ! (I :: BITS >= 16) } ; t = x >> 8 ; if t != I :: ZERO { z -= 8 ; x = t ; } t = x >> 4 ; if t != I :: ZERO { z -= 4 ; x = t ; } t = x >> 2 ; if t != I :: ZERO { z -= 2 ; x = t ; } t = x >> 1 ; if t != I :: ZERO { z - 2 } else { z - usize :: cast_from (x) } } # [doc = " Returns the number of leading binary zeros in `x`."] # [allow (dead_code)] pub fn leading_zeros_riscv < I : Int > (x : I) -> usize where usize : CastFrom < I > , { let mut x = x ; let mut z = I :: BITS ; let mut t : u32 ; const { assert ! (I :: BITS <= 64) } ; if I :: BITS >= 64 { t = ((x >= (I :: ONE << 32)) as u32) << 5 ; x >>= t ; z -= t ; } if I :: BITS >= 32 { t = ((x >= (I :: ONE << 16)) as u32) << 4 ; x >>= t ; z -= t ; } const { assert ! (I :: BITS >= 16) } ; t = ((x >= (I :: ONE << 8)) as u32) << 3 ; x >>= t ; z -= t ; t = ((x >= (I :: ONE << 4)) as u32) << 2 ; x >>= t ; z -= t ; t = ((x >= (I :: ONE << 2)) as u32) << 1 ; x >>= t ; z -= t ; t = (x >= (I :: ONE << 1)) as u32 ; x >>= t ; z -= t ; z as usize - usize :: cast_from (x) } }
};
}
