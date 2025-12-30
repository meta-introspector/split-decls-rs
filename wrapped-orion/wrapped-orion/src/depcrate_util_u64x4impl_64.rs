// Generated macro for impl_64 (impl)
macro_rules! Depcrate_util_u64x4impl_64 {
() => {
// Module: crate::util::u64x4
// Provides: {"impl_64"}
// Dependencies: {}
impl U64x4 { # [must_use] pub (crate) const fn wrapping_add (self , _rhs : Self) -> Self { Self (self . 0 . wrapping_add (_rhs . 0) , self . 1 . wrapping_add (_rhs . 1) , self . 2 . wrapping_add (_rhs . 2) , self . 3 . wrapping_add (_rhs . 3) ,) } # [must_use] pub (crate) const fn shl_1 (self) -> Self { Self (self . 1 , self . 2 , self . 3 , self . 0) } # [must_use] pub (crate) const fn shl_2 (self) -> Self { Self (self . 2 , self . 3 , self . 0 , self . 1) } # [must_use] pub (crate) const fn shl_3 (self) -> Self { Self (self . 3 , self . 0 , self . 1 , self . 2) } # [must_use] pub (crate) const fn rotate_right (self , n : u32) -> Self { Self (self . 0 . rotate_right (n) , self . 1 . rotate_right (n) , self . 2 . rotate_right (n) , self . 3 . rotate_right (n) ,) } pub (crate) fn store_into_le (self , slice_in : & mut [u8]) { debug_assert_eq ! (slice_in . len () , size_of ::< u64 > () * 4) ; let mut iter = slice_in . chunks_exact_mut (size_of :: < u64 > ()) ; iter . next () . unwrap () . copy_from_slice (& self . 0 . to_le_bytes ()) ; iter . next () . unwrap () . copy_from_slice (& self . 1 . to_le_bytes ()) ; iter . next () . unwrap () . copy_from_slice (& self . 2 . to_le_bytes ()) ; iter . next () . unwrap () . copy_from_slice (& self . 3 . to_le_bytes ()) ; } }
};
}
