macro_rules! deps {
    () => {
        CVWords!();
        CVBytes!();
    };
}

macro_rules! hash1 {
    () => {
        deps!();
        # [target_feature (enable = "simd128")] unsafe fn hash1 < const N : usize > (input : & [u8 ; N] , key : & CVWords , counter : u64 , flags : u8 , flags_start : u8 , flags_end : u8 , out : & mut CVBytes ,) { debug_assert_eq ! (N % BLOCK_LEN , 0 , "uneven blocks") ; let mut cv = * key ; let mut block_flags = flags | flags_start ; let mut slice = & input [..] ; while slice . len () >= BLOCK_LEN { if slice . len () == BLOCK_LEN { block_flags |= flags_end ; } compress_in_place (& mut cv , array_ref ! (slice , 0 , BLOCK_LEN) , BLOCK_LEN as u8 , counter , block_flags ,) ; block_flags = flags ; slice = & slice [BLOCK_LEN ..] ; } * out = unsafe { core :: mem :: transmute (cv) } ; }
    };
}

hash1!()