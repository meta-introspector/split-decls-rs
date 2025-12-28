macro_rules! deps {
    () => {
        CVWords!();
    };
}

macro_rules! compress_in_place {
    () => {
        deps!();
        # [target_feature (enable = "simd128")] pub fn compress_in_place (cv : & mut CVWords , block : & [u8 ; BLOCK_LEN] , block_len : u8 , counter : u64 , flags : u8 ,) { let [row0 , row1 , row2 , row3] = compress_pre (cv , block , block_len , counter , flags) ; unsafe { storeu (xor (row0 , row2) , cv . as_mut_ptr () . add (0) as * mut u8) ; storeu (xor (row1 , row3) , cv . as_mut_ptr () . add (4) as * mut u8) ; } }
    };
}

compress_in_place!()