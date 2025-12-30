// Generated macro for impl_180 (impl)
macro_rules! Depcrate_inflate_coreimpl_180 {
() => {
// Module: crate::inflate::core
// Provides: {"impl_180"}
// Dependencies: {}
impl Default for DecompressorOxide { # [doc = " Create a new tinfl_decompressor with all fields set to 0."] # [inline (always)] fn default () -> Self { DecompressorOxide { state : core :: State :: Start , num_bits : 0 , z_header0 : 0 , z_header1 : 0 , z_adler32 : 0 , finish : 0 , block_type : 0 , check_adler32 : 0 , dist : 0 , counter : 0 , num_extra : 0 , table_sizes : [0 ; MAX_HUFF_TABLES] , bit_buf : 0 , tables : [HuffmanTable :: new () , HuffmanTable :: new () , HuffmanTable :: new () ,] , code_size_literal : [0 ; MAX_HUFF_SYMBOLS_0] , code_size_dist : [0 ; MAX_HUFF_SYMBOLS_1] , code_size_huffman : [0 ; MAX_HUFF_SYMBOLS_2] , raw_header : [0 ; 4] , len_codes : [0 ; LEN_CODES_SIZE] , } } }
};
}
