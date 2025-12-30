// Generated macro for simd_impl (macro)
macro_rules! Depcrate_distr_integersimd_impl {
() => {
// Module: crate::distr::integer
// Provides: {"simd_impl"}
// Dependencies: {}
# [cfg (feature = "simd_support")] macro_rules ! simd_impl { ($ ($ ty : ty) ,+) => { $ (# [doc = " Requires nightly Rust and the [`simd_support`] feature"] # [doc = ""] # [doc = " [`simd_support`]: https://github.com/rust-random/rand#crate-features"] # [cfg (feature = "simd_support")] impl < const LANES : usize > Distribution < Simd <$ ty , LANES >> for StandardUniform where LaneCount < LANES >: SupportedLaneCount , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Simd <$ ty , LANES > { let mut vec = Simd :: default () ; rng . fill (vec . as_mut_array () . as_mut_slice ()) ; vec } }) + } ; }
};
}
