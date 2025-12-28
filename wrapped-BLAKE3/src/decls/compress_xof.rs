macro_rules! deps {
    () => {
        CVWords!();
    };
}

macro_rules! compress_xof {
    () => {
        deps!();
        # [target_feature (enable = "simd128")] pub fn compress_xof (cv : & CVWords , block : & [u8 ; BLOCK_LEN] , block_len : u8 , counter : u64 , flags : u8 ,) -> [u8 ; 64] { let [mut row0 , mut row1 , mut row2 , mut row3] = compress_pre (cv , block , block_len , counter , flags) ; row0 = xor (row0 , row2) ; row1 = xor (row1 , row3) ; row2 = xor (row2 , unsafe { loadu (cv . as_ptr () . add (0) as * const u8) }) ; row3 = xor (row3 , unsafe { loadu (cv . as_ptr () . add (4) as * const u8) }) ; unsafe { core :: mem :: transmute ([row0 , row1 , row2 , row3]) } }
    };
}

compress_xof!()