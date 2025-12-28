macro_rules! deps {
    () => {
        CVWords!();
    };
}

macro_rules! CompressXofFn {
    () => {
        deps!();
        type CompressXofFn = unsafe fn (cv : & CVWords , block : & [u8 ; BLOCK_LEN] , block_len : u8 , counter : u64 , flags : u8 ,) -> [u8 ; 64] ;
    };
}

CompressXofFn!();