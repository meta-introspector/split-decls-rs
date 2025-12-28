macro_rules! deps {
    () => {
        Endianness!();
        U16!();
        ReadRef!();
        Endian!();
        RelocationStateV3!();
    };
}

macro_rules! DyldCacheRelocationIteratorV3 {
    () => {
        deps!();
        # [derive (Debug)] struct DyldCacheRelocationIteratorV3 < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { data : R , endian : E , mapping_file_offset : u64 , auth_value_add : u64 , page_size : u64 , page_starts : & 'data [U16 < E >] , state : RelocationStateV3 , # [doc = " Index of the page within the mapping."] start_index : usize , # [doc = " The current offset within the mapping."] offset : u64 , }
    };
}

DyldCacheRelocationIteratorV3!()