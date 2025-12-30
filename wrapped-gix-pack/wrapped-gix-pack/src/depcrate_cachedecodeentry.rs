// Generated macro for DecodeEntry (trait)
macro_rules! Depcrate_cacheDecodeEntry {
() => {
// Module: crate::cache
// Provides: {"DecodeEntry"}
// Dependencies: {}
# [doc = " A trait to model putting objects at a given pack `offset` into a cache, and fetching them."] # [doc = ""] # [doc = " It is used to speed up [pack traversals][crate::index::File::traverse()]."] pub trait DecodeEntry { # [doc = " Store a fully decoded object at `offset` of `kind` with `compressed_size` and `data` in the cache."] # [doc = ""] # [doc = " It is up to the cache implementation whether that actually happens or not."] fn put (& mut self , pack_id : u32 , offset : u64 , data : & [u8] , kind : gix_object :: Kind , compressed_size : usize) ; # [doc = " Attempt to fetch the object at `offset` and store its decoded bytes in `out`, as previously stored with [`DecodeEntry::put()`], and return"] # [doc = " its (object `kind`, `decompressed_size`)"] fn get (& mut self , pack_id : u32 , offset : u64 , out : & mut Vec < u8 >) -> Option < (gix_object :: Kind , usize) > ; }
};
}
