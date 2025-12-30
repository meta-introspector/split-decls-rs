// Generated macro for transpose_msg_vecs (function)
macro_rules! Depcrate_wasm32_simdtranspose_msg_vecs {
() => {
// Module: crate::wasm32_simd
// Provides: {"transpose_msg_vecs"}
// Dependencies: {}
# [inline (always)] unsafe fn transpose_msg_vecs (inputs : & [* const u8 ; DEGREE] , block_offset : usize) -> [v128 ; 16] { let mut vecs = unsafe { [loadu (inputs [0] . add (block_offset + 0 * 4 * DEGREE)) , loadu (inputs [1] . add (block_offset + 0 * 4 * DEGREE)) , loadu (inputs [2] . add (block_offset + 0 * 4 * DEGREE)) , loadu (inputs [3] . add (block_offset + 0 * 4 * DEGREE)) , loadu (inputs [0] . add (block_offset + 1 * 4 * DEGREE)) , loadu (inputs [1] . add (block_offset + 1 * 4 * DEGREE)) , loadu (inputs [2] . add (block_offset + 1 * 4 * DEGREE)) , loadu (inputs [3] . add (block_offset + 1 * 4 * DEGREE)) , loadu (inputs [0] . add (block_offset + 2 * 4 * DEGREE)) , loadu (inputs [1] . add (block_offset + 2 * 4 * DEGREE)) , loadu (inputs [2] . add (block_offset + 2 * 4 * DEGREE)) , loadu (inputs [3] . add (block_offset + 2 * 4 * DEGREE)) , loadu (inputs [0] . add (block_offset + 3 * 4 * DEGREE)) , loadu (inputs [1] . add (block_offset + 3 * 4 * DEGREE)) , loadu (inputs [2] . add (block_offset + 3 * 4 * DEGREE)) , loadu (inputs [3] . add (block_offset + 3 * 4 * DEGREE)) ,] } ; let squares = mut_array_refs ! (& mut vecs , DEGREE , DEGREE , DEGREE , DEGREE) ; transpose_vecs (squares . 0) ; transpose_vecs (squares . 1) ; transpose_vecs (squares . 2) ; transpose_vecs (squares . 3) ; vecs }
};
}
