macro_rules! deps {
    () => {
        Endianness!();
        ReadRef!();
        Endian!();
        DyldCacheMappingVersionIterator!();
    };
}

macro_rules! DyldCacheMappingIterator {
    () => {
        deps!();
        # [doc = " An iterator over all the mappings for one subcache in a dyld shared cache."] # [derive (Debug)] pub struct DyldCacheMappingIterator < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { endian : E , data : R , iter : DyldCacheMappingVersionIterator < 'data , E > , }
    };
}

DyldCacheMappingIterator!()