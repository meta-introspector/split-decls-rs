macro_rules! deps {
    () => {
        ReadRef!();
        DyldCacheImageInfo!();
        Endian!();
        Endianness!();
        DyldCache!();
    };
}

macro_rules! DyldCacheImageIterator {
    () => {
        deps!();
        # [doc = " An iterator over all the images (dylibs) in the dyld shared cache."] # [derive (Debug)] pub struct DyldCacheImageIterator < 'data , 'cache , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { cache : & 'cache DyldCache < 'data , E , R > , iter : slice :: Iter < 'data , macho :: DyldCacheImageInfo < E > > , }
    };
}

DyldCacheImageIterator!();