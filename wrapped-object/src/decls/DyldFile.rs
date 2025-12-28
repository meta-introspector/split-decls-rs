macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        ReadRef!();
        DyldCacheMappingSlice!();
    };
}

macro_rules! DyldFile {
    () => {
        deps!();
        # [doc = " The data for one file in the cache."] # [derive (Debug)] struct DyldFile < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { data : R , mappings : DyldCacheMappingSlice < 'data , E > , }
    };
}

DyldFile!();