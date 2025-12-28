macro_rules! Gidx {
    () => {
        # [doc = " Data-independent indexing."] struct Gidx { block : [u64 ; 128] , addresses : [u64 ; 128] , segment_length : u32 , offset : u32 , }
    };
}

Gidx!()