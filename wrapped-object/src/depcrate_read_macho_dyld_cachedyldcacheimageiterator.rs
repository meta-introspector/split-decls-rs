// Generated macro for DyldCacheImageIterator (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheImageIterator {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheImageIterator"}
// Dependencies: {}
# [doc = " An iterator over all the images (dylibs) in the dyld shared cache."] # [derive (Debug)] pub struct DyldCacheImageIterator < 'data , 'cache , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { cache : & 'cache DyldCache < 'data , E , R > , iter : slice :: Iter < 'data , macho :: DyldCacheImageInfo < E > > , }
};
}
