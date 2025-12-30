// Generated macro for DyldCacheImage (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheImage {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheImage"}
// Dependencies: {}
# [doc = " One image (dylib) from inside the dyld shared cache."] # [derive (Debug)] pub struct DyldCacheImage < 'data , 'cache , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { pub (crate) cache : & 'cache DyldCache < 'data , E , R > , image_info : & 'data macho :: DyldCacheImageInfo < E > , }
};
}
