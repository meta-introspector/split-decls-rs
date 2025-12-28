macro_rules! deps {
    () => {
        IncrementCounter!();
        CVWords!();
    };
}

macro_rules! hash4 {
    () => {
        deps!();
        # [target_feature (enable = "simd128")] pub unsafe fn hash4 (inputs : & [* const u8 ; DEGREE] , blocks : usize , key : & CVWords , counter : u64 , increment_counter : IncrementCounter , flags : u8 , flags_start : u8 , flags_end : u8 , out : & mut [u8 ; DEGREE * OUT_LEN] ,) { let mut h_vecs = [set1 (key [0]) , set1 (key [1]) , set1 (key [2]) , set1 (key [3]) , set1 (key [4]) , set1 (key [5]) , set1 (key [6]) , set1 (key [7]) ,] ; let (counter_low_vec , counter_high_vec) = load_counters (counter , increment_counter) ; let mut block_flags = flags | flags_start ; for block in 0 .. blocks { if block + 1 == blocks { block_flags |= flags_end ; } let block_len_vec = set1 (BLOCK_LEN as u32) ; let block_flags_vec = set1 (block_flags as u32) ; let msg_vecs = unsafe { transpose_msg_vecs (inputs , block * BLOCK_LEN) } ; let mut v = [h_vecs [0] , h_vecs [1] , h_vecs [2] , h_vecs [3] , h_vecs [4] , h_vecs [5] , h_vecs [6] , h_vecs [7] , set1 (IV [0]) , set1 (IV [1]) , set1 (IV [2]) , set1 (IV [3]) , counter_low_vec , counter_high_vec , block_len_vec , block_flags_vec ,] ; round (& mut v , & msg_vecs , 0) ; round (& mut v , & msg_vecs , 1) ; round (& mut v , & msg_vecs , 2) ; round (& mut v , & msg_vecs , 3) ; round (& mut v , & msg_vecs , 4) ; round (& mut v , & msg_vecs , 5) ; round (& mut v , & msg_vecs , 6) ; h_vecs [0] = xor (v [0] , v [8]) ; h_vecs [1] = xor (v [1] , v [9]) ; h_vecs [2] = xor (v [2] , v [10]) ; h_vecs [3] = xor (v [3] , v [11]) ; h_vecs [4] = xor (v [4] , v [12]) ; h_vecs [5] = xor (v [5] , v [13]) ; h_vecs [6] = xor (v [6] , v [14]) ; h_vecs [7] = xor (v [7] , v [15]) ; block_flags = flags ; } let squares = mut_array_refs ! (& mut h_vecs , DEGREE , DEGREE) ; transpose_vecs (squares . 0) ; transpose_vecs (squares . 1) ; unsafe { storeu (h_vecs [0] , out . as_mut_ptr () . add (0 * 4 * DEGREE)) ; storeu (h_vecs [4] , out . as_mut_ptr () . add (1 * 4 * DEGREE)) ; storeu (h_vecs [1] , out . as_mut_ptr () . add (2 * 4 * DEGREE)) ; storeu (h_vecs [5] , out . as_mut_ptr () . add (3 * 4 * DEGREE)) ; storeu (h_vecs [2] , out . as_mut_ptr () . add (4 * 4 * DEGREE)) ; storeu (h_vecs [6] , out . as_mut_ptr () . add (5 * 4 * DEGREE)) ; storeu (h_vecs [3] , out . as_mut_ptr () . add (6 * 4 * DEGREE)) ; storeu (h_vecs [7] , out . as_mut_ptr () . add (7 * 4 * DEGREE)) ; } }
    };
}

hash4!();