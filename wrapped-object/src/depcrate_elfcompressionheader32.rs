// Generated macro for CompressionHeader32 (struct)
macro_rules! Depcrate_elfCompressionHeader32 {
() => {
// Module: crate::elf
// Provides: {"CompressionHeader32"}
// Dependencies: {}
# [doc = " Section compression header."] # [doc = ""] # [doc = " Used when `SHF_COMPRESSED` is set."] # [doc = ""] # [doc = " Note: this type currently allows for misaligned headers, but that may be"] # [doc = " changed in a future version."] # [derive (Debug , Default , Clone , Copy)] # [repr (C)] pub struct CompressionHeader32 < E : Endian > { # [doc = " Compression format. One of the `ELFCOMPRESS_*` values."] pub ch_type : U32Bytes < E > , # [doc = " Uncompressed data size."] pub ch_size : U32Bytes < E > , # [doc = " Uncompressed data alignment."] pub ch_addralign : U32Bytes < E > , }
};
}
