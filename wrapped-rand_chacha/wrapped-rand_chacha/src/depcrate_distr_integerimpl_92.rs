// Generated macro for impl_92 (impl)
macro_rules! Depcrate_distr_integerimpl_92 {
() => {
// Module: crate::distr::integer
// Provides: {"impl_92"}
// Dependencies: {}
# [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , feature = "simd_support"))] impl Distribution < __m512i > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> __m512i { let mut buf = [0_u8 ; core :: mem :: size_of :: < __m512i > ()] ; rng . fill_bytes (& mut buf) ; unsafe { core :: mem :: transmute (buf) } } }
};
}
