// Generated macro for macro_248 (macro)
macro_rules! Depcrate_x86_64macro_248 {
() => {
// Module: crate::x86_64
// Provides: {"macro_248"}
// Dependencies: {}
zerocopy :: cryptocorrosion_derive_traits ! { # [repr (C)] # [doc = " Generic wrapper for unparameterized storage of any of the possible impls."] # [doc = " Converting into and out of this type should be essentially free, although it may be more"] # [doc = " aligned than a particular impl requires."] # [allow (non_camel_case_types)] # [derive (Copy , Clone)] pub union vec128_storage { u32x4 : [u32 ; 4] , u64x2 : [u64 ; 2] , u128x1 : [u128 ; 1] , sse2 : __m128i , } }
};
}
