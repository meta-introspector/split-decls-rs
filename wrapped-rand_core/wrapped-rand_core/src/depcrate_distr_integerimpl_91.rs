// Generated macro for impl_91 (impl)
macro_rules! Depcrate_distr_integerimpl_91 {
() => {
// Module: crate::distr::integer
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] impl Distribution < __m256i > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> __m256i { let mut buf = [0_u8 ; core :: mem :: size_of :: < __m256i > ()] ; rng . fill_bytes (& mut buf) ; unsafe { core :: mem :: transmute (buf) } } }
};
}
