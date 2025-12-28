macro_rules! deps {
    () => {
        U16!();
        Endianness!();
        Endian!();
        RelocationStateV2!();
        ReadRef!();
    };
}

macro_rules! DyldCacheRelocationIteratorV2 {
    () => {
        deps!();
        # [derive (Debug)] struct DyldCacheRelocationIteratorV2 < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { data : R , endian : E , mapping_file_offset : u64 , page_size : u64 , delta_mask : u64 , delta_shift : u32 , value_add : u64 , page_starts : & 'data [U16 < E >] , page_extras : & 'data [U16 < E >] , state : RelocationStateV2 , # [doc = " The next index within page_starts."] start_index : usize , # [doc = " The next index within page_extras."] extra_index : usize , # [doc = " The current page offset within the mapping."] page_offset : u64 , # [doc = " The offset of the next linked list entry within the page."] offset : u64 , }
    };
}

DyldCacheRelocationIteratorV2!();