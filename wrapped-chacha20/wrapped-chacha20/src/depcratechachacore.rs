// Generated macro for ChaChaCore (struct)
macro_rules! DepcrateChaChaCore {
() => {
// Module: crate
// Provides: {"ChaChaCore"}
// Dependencies: {}
# [doc = " The ChaCha core function."] pub struct ChaChaCore < R : Rounds , V : Variant > { # [doc = " Internal state of the core function"] # [cfg (any (feature = "cipher" , feature = "rng"))] state : [u32 ; STATE_WORDS] , # [doc = " CPU target feature tokens"] # [allow (dead_code)] tokens : Tokens , # [doc = " Number of rounds to perform and the cipher variant"] _pd : PhantomData < (R , V) > , }
};
}
