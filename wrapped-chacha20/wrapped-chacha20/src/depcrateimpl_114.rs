// Generated macro for impl_114 (impl)
macro_rules! Depcrateimpl_114 {
() => {
// Module: crate
// Provides: {"impl_114"}
// Dependencies: {}
# [cfg (feature = "cipher")] impl < R : Rounds , V : Variant > StreamCipherCore for ChaChaCore < R , V > { # [inline (always)] fn remaining_blocks (& self) -> Option < usize > { V :: remaining_blocks (self . get_block_pos ()) } fn process_with_backend (& mut self , f : impl cipher :: StreamCipherClosure < BlockSize = Self :: BlockSize > ,) { cfg_if ! { if # [cfg (chacha20_force_soft)] { f . call (& mut backends :: soft :: Backend (self)) ; } else if # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { cfg_if ! { if # [cfg (chacha20_force_avx2)] { unsafe { backends :: avx2 :: inner ::< R , _ , V > (& mut self . state , f) ; } } else if # [cfg (chacha20_force_sse2)] { unsafe { backends :: sse2 :: inner ::< R , _ , V > (& mut self . state , f) ; } } else { let (avx2_token , sse2_token) = self . tokens ; if avx2_token . get () { unsafe { backends :: avx2 :: inner ::< R , _ , V > (& mut self . state , f) ; } } else if sse2_token . get () { unsafe { backends :: sse2 :: inner ::< R , _ , V > (& mut self . state , f) ; } } else { f . call (& mut backends :: soft :: Backend (self)) ; } } } } else if # [cfg (all (target_arch = "aarch64" , target_feature = "neon"))] { unsafe { backends :: neon :: inner ::< R , _ , V > (& mut self . state , f) ; } } else { f . call (& mut backends :: soft :: Backend (self)) ; } } } }
};
}
