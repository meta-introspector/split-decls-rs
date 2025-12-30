// Generated macro for PmacState (struct)
macro_rules! Depcrate_block_apiPmacState {
() => {
// Module: crate::block_api
// Provides: {"PmacState"}
// Dependencies: {}
# [derive (Clone)] struct PmacState < C : PmacCipher , const LC_SIZE : usize > { counter : usize , l_inv : Block < C > , l_cache : [Block < C > ; LC_SIZE] , tag : Block < C > , offset : Block < C > , }
};
}
