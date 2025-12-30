// Generated macro for Lzma2Options (struct)
macro_rules! Depcrate_enc_lzma2_writerLzma2Options {
() => {
// Module: crate::enc::lzma2_writer
// Provides: {"Lzma2Options"}
// Dependencies: {}
# [doc = " Options for LZMA2 compression."] # [derive (Default , Debug , Clone)] pub struct Lzma2Options { # [doc = " LZMA compression options."] pub lzma_options : LzmaOptions , # [doc = " The size of each independent chunk in bytes."] # [doc = " If not set, the whole data will be written as one chunk."] # [doc = " Will get clamped to be at least the dict size to not waste memory."] pub chunk_size : Option < NonZeroU64 > , }
};
}
