// Generated macro for DyldCacheRelocationIteratorV2 (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheRelocationIteratorV2 {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheRelocationIteratorV2"}
// Dependencies: {}
# [derive (Debug)] struct DyldCacheRelocationIteratorV2 < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { data : R , endian : E , mapping_file_offset : u64 , page_size : u64 , delta_mask : u64 , delta_shift : u32 , value_add : u64 , page_starts : & 'data [U16 < E >] , page_extras : & 'data [U16 < E >] , state : RelocationStateV2 , # [doc = " The next index within page_starts."] start_index : usize , # [doc = " The next index within page_extras."] extra_index : usize , # [doc = " The current page offset within the mapping."] page_offset : u64 , # [doc = " The offset of the next linked list entry within the page."] offset : u64 , }
};
}
