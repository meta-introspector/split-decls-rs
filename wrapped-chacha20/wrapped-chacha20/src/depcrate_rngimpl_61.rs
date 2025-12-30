// Generated macro for impl_61 (impl)
macro_rules! Depcrate_rngimpl_61 {
() => {
// Module: crate::rng
// Provides: {"impl_61"}
// Dependencies: {}
impl < R : Rounds , V : Variant > ChaChaCore < R , V > { # [doc = " Generates 4 blocks in parallel with avx2 & neon, but merely fills"] # [doc = " 4 blocks with sse2 & soft"] fn generate (& mut self , buffer : & mut [u32 ; 64]) { cfg_if ! { if # [cfg (chacha20_force_soft)] { backends :: soft :: Backend (self) . gen_ks_blocks (buffer) ; } else if # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { cfg_if ! { if # [cfg (chacha20_force_avx2)] { unsafe { backends :: avx2 :: rng_inner ::< R , V > (self , buffer) ; } } else if # [cfg (chacha20_force_sse2)] { unsafe { backends :: sse2 :: rng_inner ::< R , V > (self , buffer) ; } } else { let (avx2_token , sse2_token) = self . tokens ; if avx2_token . get () { unsafe { backends :: avx2 :: rng_inner ::< R , V > (self , buffer) ; } } else if sse2_token . get () { unsafe { backends :: sse2 :: rng_inner ::< R , V > (self , buffer) ; } } else { backends :: soft :: Backend (self) . gen_ks_blocks (buffer) ; } } } } else if # [cfg (all (target_arch = "aarch64" , target_feature = "neon"))] { unsafe { backends :: neon :: rng_inner ::< R , V > (self , buffer) ; } } else { backends :: soft :: Backend (self) . gen_ks_blocks (buffer) ; } } } }
};
}
