macro_rules! deps {
    () => {
        Never!();
        DecodeEntry!();
        Kind!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl DecodeEntry for Never { fn put (& mut self , _pack_id : u32 , _offset : u64 , _data : & [u8] , _kind : gix_object :: Kind , _compressed_size : usize) { } fn get (& mut self , _pack_id : u32 , _offset : u64 , _out : & mut Vec < u8 >) -> Option < (gix_object :: Kind , usize) > { None } }
    };
}

impl_34!()