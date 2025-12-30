// Generated macro for DyldCacheRelocationIteratorV5 (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheRelocationIteratorV5 {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheRelocationIteratorV5"}
// Dependencies: {}
# [derive (Debug)] struct DyldCacheRelocationIteratorV5 < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { data : R , endian : E , mapping_file_offset : u64 , page_size : u64 , value_add : u64 , page_starts : & 'data [U16 < E >] , state : RelocationStateV5 , # [doc = " The next index within page_starts."] start_index : usize , # [doc = " The current offset within the mapping."] offset : u64 , }
};
}
