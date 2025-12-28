macro_rules! deps {
    () => {
        Section!();
        U64Bytes!();
        Note!();
        Endian!();
        U32Bytes!();
    };
}

macro_rules! CompressionHeader64 {
    () => {
        deps!();
        # [doc = " Section compression header."] # [doc = ""] # [doc = " Used when `SHF_COMPRESSED` is set."] # [doc = ""] # [doc = " Note: this type currently allows for misaligned headers, but that may be"] # [doc = " changed in a future version."] # [derive (Debug , Default , Clone , Copy)] # [repr (C)] pub struct CompressionHeader64 < E : Endian > { # [doc = " Compression format. One of the `ELFCOMPRESS_*` values."] pub ch_type : U32Bytes < E > , # [doc = " Reserved."] pub ch_reserved : U32Bytes < E > , # [doc = " Uncompressed data size."] pub ch_size : U64Bytes < E > , # [doc = " Uncompressed data alignment."] pub ch_addralign : U64Bytes < E > , }
    };
}

CompressionHeader64!();