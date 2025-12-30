// Generated macro for compress_xof (function)
macro_rules! Depcrate_portablecompress_xof {
() => {
// Module: crate::portable
// Provides: {"compress_xof"}
// Dependencies: {}
pub fn compress_xof (cv : & CVWords , block : & [u8 ; BLOCK_LEN] , block_len : u8 , counter : u64 , flags : u8 ,) -> [u8 ; 64] { let mut state = compress_pre (cv , block , block_len , counter , flags) ; state [0] ^= state [8] ; state [1] ^= state [9] ; state [2] ^= state [10] ; state [3] ^= state [11] ; state [4] ^= state [12] ; state [5] ^= state [13] ; state [6] ^= state [14] ; state [7] ^= state [15] ; state [8] ^= cv [0] ; state [9] ^= cv [1] ; state [10] ^= cv [2] ; state [11] ^= cv [3] ; state [12] ^= cv [4] ; state [13] ^= cv [5] ; state [14] ^= cv [6] ; state [15] ^= cv [7] ; crate :: platform :: le_bytes_from_words_64 (& state) }
};
}
