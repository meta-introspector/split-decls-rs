// Generated macro for impl_112 (impl)
macro_rules! Depcrateimpl_112 {
() => {
// Module: crate
// Provides: {"impl_112"}
// Dependencies: {}
impl < R : Rounds , V : Variant > ChaChaCore < R , V > { # [doc = " Constructs a ChaChaCore with the specified key, iv, and amount of rounds."] # [doc = " You must ensure that the iv is of the correct size when using this method"] # [doc = " directly."] # [cfg (any (feature = "cipher" , feature = "rng"))] fn new (key : & [u8 ; 32] , iv : & [u8]) -> Self { let mut state = [0u32 ; STATE_WORDS] ; let ctr_size = size_of :: < V :: Counter > () / size_of :: < u32 > () ; let (const_dst , state_rem) = state . split_at_mut (4) ; let (key_dst , state_rem) = state_rem . split_at_mut (8) ; let (_ctr_dst , iv_dst) = state_rem . split_at_mut (ctr_size) ; const_dst . copy_from_slice (& CONSTANTS) ; for (src , dst) in key . chunks_exact (4) . zip (key_dst) { * dst = u32 :: from_le_bytes (src . try_into () . unwrap ()) ; } assert_eq ! (size_of_val (iv_dst) , size_of_val (iv)) ; for (src , dst) in iv . chunks_exact (4) . zip (iv_dst) { * dst = u32 :: from_le_bytes (src . try_into () . unwrap ()) ; } cfg_if ! { if # [cfg (chacha20_force_soft)] { let tokens = () ; } else if # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { cfg_if ! { if # [cfg (chacha20_force_avx2)] { let tokens = () ; } else if # [cfg (chacha20_force_sse2)] { let tokens = () ; } else { let tokens = (avx2_cpuid :: init () , sse2_cpuid :: init ()) ; } } } else { let tokens = () ; } } Self { state , tokens , _pd : PhantomData , } } }
};
}
