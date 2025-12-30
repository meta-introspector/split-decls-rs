// Generated macro for Algorithm (struct)
macro_rules! Depcrate_digestAlgorithm {
() => {
// Module: crate::digest
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " A digest algorithm."] pub struct Algorithm { output_len : OutputLen , chaining_len : usize , block_len : BlockLen , # [doc = " `block_data_order` processes all the full blocks of data in `data`. It"] # [doc = " returns the number of bytes processed and the unprocessed data, which"] # [doc = " is guaranteed to be less than `block_len` bytes long."] block_data_order : for < 'd > fn (state : & mut DynState , data : & 'd [u8] , cpu_features : cpu :: Features ,) -> (usize , & 'd [u8]) , initial_state : DynInitialState , id : AlgorithmID , }
};
}
