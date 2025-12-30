// Generated macro for LzmaOptions (struct)
macro_rules! Depcrate_enc_lzma2_writerLzmaOptions {
() => {
// Module: crate::enc::lzma2_writer
// Provides: {"LzmaOptions"}
// Dependencies: {}
# [doc = " Encoder settings when compressing with LZMA and LZMA2."] # [derive (Debug , Clone)] pub struct LzmaOptions { # [doc = " Dictionary size in bytes."] pub dict_size : u32 , # [doc = " Number of literal context bits (0-8)."] pub lc : u32 , # [doc = " Number of literal position bits (0-4)."] pub lp : u32 , # [doc = " Number of position bits (0-4)."] pub pb : u32 , # [doc = " Compression mode."] pub mode : EncodeMode , # [doc = " Match finder nice length."] pub nice_len : u32 , # [doc = " Match finder type."] pub mf : MfType , # [doc = " Match finder depth limit."] pub depth_limit : i32 , # [doc = " Preset dictionary data."] pub preset_dict : Option < Vec < u8 > > , }
};
}
