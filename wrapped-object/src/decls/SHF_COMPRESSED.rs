macro_rules! deps {
    () => {
        Section!();
        CompressionHeader!();
    };
}

macro_rules! SHF_COMPRESSED {
    () => {
        deps!();
        # [doc = " Section is compressed."] # [doc = ""] # [doc = " Compressed sections begin with one of the `CompressionHeader*` headers."] pub const SHF_COMPRESSED : u32 = 1 << 11 ;
    };
}

SHF_COMPRESSED!();