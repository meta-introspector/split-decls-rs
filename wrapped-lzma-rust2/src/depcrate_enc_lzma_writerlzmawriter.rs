// Generated macro for LzmaWriter (struct)
macro_rules! Depcrate_enc_lzma_writerLzmaWriter {
() => {
// Module: crate::enc::lzma_writer
// Provides: {"LzmaWriter"}
// Dependencies: {}
# [doc = " A single-threaded LZMA compressor."] pub struct LzmaWriter < W : Write > { rc : RangeEncoder < W > , lzma : LzmaEncoder , use_end_marker : bool , current_uncompressed_size : u64 , expected_uncompressed_size : Option < u64 > , props : u8 , mode : LzmaEncoderModes , }
};
}
