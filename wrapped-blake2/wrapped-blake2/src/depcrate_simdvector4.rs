// Generated macro for Vector4 (trait)
macro_rules! Depcrate_simdVector4 {
() => {
// Module: crate::simd
// Provides: {"Vector4"}
// Dependencies: {}
pub (crate) trait Vector4 < T > : Copy { fn gather (src : & [T] , i0 : usize , i1 : usize , i2 : usize , i3 : usize) -> Self ; # [allow (clippy :: wrong_self_convention)] fn from_le (self) -> Self ; fn to_le (self) -> Self ; fn wrapping_add (self , rhs : Self) -> Self ; fn rotate_right_const (self , n : u32) -> Self ; fn shuffle_left_1 (self) -> Self ; fn shuffle_left_2 (self) -> Self ; fn shuffle_left_3 (self) -> Self ; # [inline (always)] fn shuffle_right_1 (self) -> Self { self . shuffle_left_3 () } # [inline (always)] fn shuffle_right_2 (self) -> Self { self . shuffle_left_2 () } # [inline (always)] fn shuffle_right_3 (self) -> Self { self . shuffle_left_1 () } }
};
}
