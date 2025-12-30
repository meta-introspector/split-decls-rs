// Generated macro for Lzma2Writer (struct)
macro_rules! Depcrate_enc_lzma2_writerLzma2Writer {
() => {
// Module: crate::enc::lzma2_writer
// Provides: {"Lzma2Writer"}
// Dependencies: {}
# [doc = " A single-threaded LZMA2 compressor."] pub struct Lzma2Writer < W : Write > { inner : W , rc : RangeEncoder < RangeEncoderBuffer > , lzma : LzmaEncoder , mode : LzmaEncoderModes , dict_reset_needed : bool , state_reset_needed : bool , props_needed : bool , pending_size : u32 , chunk_size : Option < u64 > , uncompressed_size : u64 , force_independent_chunk : bool , options : Lzma2Options , }
};
}
