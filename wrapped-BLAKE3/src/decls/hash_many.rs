macro_rules! deps {
    () => {
        IncrementCounter!();
        CVWords!();
    };
}

macro_rules! hash_many {
    () => {
        deps!();
        # [target_feature (enable = "simd128")] pub unsafe fn hash_many < const N : usize > (mut inputs : & [& [u8 ; N]] , key : & CVWords , mut counter : u64 , increment_counter : IncrementCounter , flags : u8 , flags_start : u8 , flags_end : u8 , mut out : & mut [u8] ,) { debug_assert ! (out . len () >= inputs . len () * OUT_LEN , "out too short") ; while inputs . len () >= DEGREE && out . len () >= DEGREE * OUT_LEN { let input_ptrs : & [* const u8 ; DEGREE] = unsafe { & * (inputs . as_ptr () as * const [* const u8 ; DEGREE]) } ; let blocks = N / BLOCK_LEN ; unsafe { hash4 (input_ptrs , blocks , key , counter , increment_counter , flags , flags_start , flags_end , array_mut_ref ! (out , 0 , DEGREE * OUT_LEN) ,) ; } if increment_counter . yes () { counter += DEGREE as u64 ; } inputs = & inputs [DEGREE ..] ; out = & mut out [DEGREE * OUT_LEN ..] ; } for (& input , output) in inputs . iter () . zip (out . chunks_exact_mut (OUT_LEN)) { unsafe { hash1 (input , key , counter , flags , flags_start , flags_end , array_mut_ref ! (output , 0 , OUT_LEN) ,) ; } if increment_counter . yes () { counter += 1 ; } } }
    };
}

hash_many!();