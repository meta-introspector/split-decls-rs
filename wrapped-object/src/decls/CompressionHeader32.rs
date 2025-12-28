macro_rules! deps {
    () => {
        Endian!();
        U32Bytes!();
        Note!();
        Section!();
    };
}

macro_rules! CompressionHeader32 {
    () => {
        deps!();
        # [doc = " Section compression header."] # [doc = ""] # [doc = " Used when `SHF_COMPRESSED` is set."] # [doc = ""] # [doc = " Note: this type currently allows for misaligned headers, but that may be"] # [doc = " changed in a future version."] # [derive (Debug , Default , Clone , Copy)] # [repr (C)] pub struct CompressionHeader32 < E : Endian > { # [doc = " Compression format. One of the `ELFCOMPRESS_*` values."] pub ch_type : U32Bytes < E > , # [doc = " Uncompressed data size."] pub ch_size : U32Bytes < E > , # [doc = " Uncompressed data alignment."] pub ch_addralign : U32Bytes < E > , }
    };
}

CompressionHeader32!();