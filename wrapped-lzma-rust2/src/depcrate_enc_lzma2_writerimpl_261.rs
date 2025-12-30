// Generated macro for impl_261 (impl)
macro_rules! Depcrate_enc_lzma2_writerimpl_261 {
() => {
// Module: crate::enc::lzma2_writer
// Provides: {"impl_261"}
// Dependencies: {}
impl Lzma2Options { # [doc = " Create options with specific preset."] pub fn with_preset (preset : u32) -> Self { Self { lzma_options : LzmaOptions :: with_preset (preset) , chunk_size : None , } } # [doc = " Set the chunk size (None means a single chunk, which is the default)."] # [doc = " Chunk size will be clamped to be at least the dictionary size."] pub fn set_chunk_size (& mut self , chunk_size : Option < NonZeroU64 >) { self . chunk_size = chunk_size ; } }
};
}
