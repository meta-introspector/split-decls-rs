// Generated macro for compress_pre (function)
macro_rules! Depcrate_portablecompress_pre {
() => {
// Module: crate::portable
// Provides: {"compress_pre"}
// Dependencies: {}
# [inline (always)] fn compress_pre (cv : & CVWords , block : & [u8 ; BLOCK_LEN] , block_len : u8 , counter : u64 , flags : u8 ,) -> [u32 ; 16] { let block_words = crate :: platform :: words_from_le_bytes_64 (block) ; let mut state = [cv [0] , cv [1] , cv [2] , cv [3] , cv [4] , cv [5] , cv [6] , cv [7] , IV [0] , IV [1] , IV [2] , IV [3] , counter_low (counter) , counter_high (counter) , block_len as u32 , flags as u32 ,] ; round (& mut state , & block_words , 0) ; round (& mut state , & block_words , 1) ; round (& mut state , & block_words , 2) ; round (& mut state , & block_words , 3) ; round (& mut state , & block_words , 4) ; round (& mut state , & block_words , 5) ; round (& mut state , & block_words , 6) ; state }
};
}
