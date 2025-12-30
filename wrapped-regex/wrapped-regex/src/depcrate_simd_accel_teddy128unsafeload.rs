// Generated macro for UnsafeLoad (trait)
macro_rules! Depcrate_simd_accel_teddy128UnsafeLoad {
() => {
// Module: crate::simd_accel::teddy128
// Provides: {"UnsafeLoad"}
// Dependencies: {}
# [doc = " UnsafeLoad permits loading data into a SIMD vector without bounds checks."] # [doc = ""] # [doc = " Ideally, this would be part of the `simd` crate, or even better, we could"] # [doc = " figure out how to do it without `unsafe` at all."] trait UnsafeLoad { type Elem ; # [doc = " load_unchecked creates a new SIMD vector from the elements in `slice`"] # [doc = " starting at `offset`. `slice` must have at least the number of elements"] # [doc = " required to fill a SIMD vector."] unsafe fn load_unchecked (slice : & [Self :: Elem] , offset : usize) -> Self ; }
};
}
