macro_rules! deps {
    () => {
        Endianness!();
        ReadRef!();
        Endian!();
        U16!();
        RelocationStateV5!();
    };
}

macro_rules! DyldCacheRelocationIteratorV5 {
    () => {
        deps!();
        # [derive (Debug)] struct DyldCacheRelocationIteratorV5 < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { data : R , endian : E , mapping_file_offset : u64 , page_size : u64 , value_add : u64 , page_starts : & 'data [U16 < E >] , state : RelocationStateV5 , # [doc = " The next index within page_starts."] start_index : usize , # [doc = " The current offset within the mapping."] offset : u64 , }
    };
}

DyldCacheRelocationIteratorV5!()