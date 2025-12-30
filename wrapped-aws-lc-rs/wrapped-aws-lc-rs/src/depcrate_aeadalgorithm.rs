// Generated macro for Algorithm (struct)
macro_rules! Depcrate_aeadAlgorithm {
() => {
// Module: crate::aead
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " An AEAD Algorithm."] pub struct Algorithm { init : fn (key : & [u8] , tag_len : usize) -> Result < AeadCtx , Unspecified > , key_len : usize , id : AlgorithmID , max_input_len : u64 , }
};
}
