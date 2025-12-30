// Generated macro for DyldCache (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCache {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCache"}
// Dependencies: {}
# [doc = " A parsed representation of the dyld shared cache."] # [derive (Debug)] pub struct DyldCache < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { endian : E , data : R , # [doc = " The first entry is the main cache file, and the rest are subcaches."] files : Vec < DyldFile < 'data , E , R > > , images : & 'data [macho :: DyldCacheImageInfo < E >] , arch : Architecture , }
};
}
