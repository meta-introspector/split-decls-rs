macro_rules! deps {
    () => {
        DyldCacheRelocationIteratorVersion!();
        Endian!();
        ReadRef!();
        Endianness!();
    };
}

macro_rules! DyldCacheRelocationIterator {
    () => {
        deps!();
        # [doc = " An iterator over relocations in a mapping"] # [derive (Debug)] pub struct DyldCacheRelocationIterator < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { version : DyldCacheRelocationIteratorVersion < 'data , E , R > , }
    };
}

DyldCacheRelocationIterator!();