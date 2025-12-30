// Generated macro for compress_in_place (function)
macro_rules! Depcrate_portablecompress_in_place {
() => {
// Module: crate::portable
// Provides: {"compress_in_place"}
// Dependencies: {}
pub fn compress_in_place (cv : & mut CVWords , block : & [u8 ; BLOCK_LEN] , block_len : u8 , counter : u64 , flags : u8 ,) { let state = compress_pre (cv , block , block_len , counter , flags) ; cv [0] = state [0] ^ state [8] ; cv [1] = state [1] ^ state [9] ; cv [2] = state [2] ^ state [10] ; cv [3] = state [3] ^ state [11] ; cv [4] = state [4] ^ state [12] ; cv [5] = state [5] ^ state [13] ; cv [6] = state [6] ^ state [14] ; cv [7] = state [7] ^ state [15] ; }
};
}
