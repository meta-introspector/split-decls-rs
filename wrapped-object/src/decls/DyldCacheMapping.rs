macro_rules! deps {
    () => {
        Endianness!();
        DyldCacheMappingVersion!();
        ReadRef!();
        Endian!();
    };
}

macro_rules! DyldCacheMapping {
    () => {
        deps!();
        # [doc = " Information about a mapping."] # [derive (Clone , Copy)] pub struct DyldCacheMapping < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { endian : E , data : R , info : DyldCacheMappingVersion < 'data , E > , }
    };
}

DyldCacheMapping!();