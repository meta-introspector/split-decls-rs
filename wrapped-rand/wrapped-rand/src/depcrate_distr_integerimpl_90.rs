// Generated macro for impl_90 (impl)
macro_rules! Depcrate_distr_integerimpl_90 {
() => {
// Module: crate::distr::integer
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] impl Distribution < __m128i > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> __m128i { let mut buf = [0_u8 ; core :: mem :: size_of :: < __m128i > ()] ; rng . fill_bytes (& mut buf) ; unsafe { core :: mem :: transmute (buf) } } }
};
}
