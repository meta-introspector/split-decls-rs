macro_rules! deps {
    () => {
        CVWords!();
    };
}

macro_rules! XofManyFunction {
    () => {
        deps!();
        # [allow (unused)] type XofManyFunction = unsafe fn (cv : & CVWords , block : & [u8 ; BLOCK_LEN] , block_len : u8 , counter : u64 , flags : u8 , out : & mut [u8] ,) ;
    };
}

XofManyFunction!()