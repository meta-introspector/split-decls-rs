macro_rules! deps {
    () => {
        CVWords!();
    };
}

macro_rules! CompressInPlaceFn {
    () => {
        deps!();
        type CompressInPlaceFn = unsafe fn (cv : & mut CVWords , block : & [u8 ; BLOCK_LEN] , block_len : u8 , counter : u64 , flags : u8) ;
    };
}

CompressInPlaceFn!()