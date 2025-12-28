macro_rules! deps {
    () => {
        ChunkState!();
        OutputReader!();
        Output!();
        Hash!();
        Platform!();
        Hasher!();
    };
}

macro_rules! test_zeroize {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] # [test] fn test_zeroize () { use zeroize :: Zeroize ; let mut hash = crate :: Hash ([42 ; 32]) ; hash . zeroize () ; assert_eq ! (hash . 0 , [0u8 ; 32]) ; let mut hasher = crate :: Hasher { chunk_state : crate :: ChunkState { cv : [42 ; 8] , chunk_counter : 42 , buf : [42 ; 64] , buf_len : 42 , blocks_compressed : 42 , flags : 42 , platform : crate :: Platform :: Portable , } , initial_chunk_counter : 42 , key : [42 ; 8] , cv_stack : [[42 ; 32] ; { crate :: MAX_DEPTH + 1 }] . into () , } ; hasher . zeroize () ; assert_eq ! (hasher . chunk_state . cv , [0 ; 8]) ; assert_eq ! (hasher . chunk_state . chunk_counter , 0) ; assert_eq ! (hasher . chunk_state . buf , [0 ; 64]) ; assert_eq ! (hasher . chunk_state . buf_len , 0) ; assert_eq ! (hasher . chunk_state . blocks_compressed , 0) ; assert_eq ! (hasher . chunk_state . flags , 0) ; assert ! (matches ! (hasher . chunk_state . platform , crate :: Platform :: Portable)) ; assert_eq ! (hasher . initial_chunk_counter , 0) ; assert_eq ! (hasher . key , [0 ; 8]) ; assert_eq ! (&* hasher . cv_stack , & [[0u8 ; 32] ; 0]) ; let mut output_reader = crate :: OutputReader { inner : crate :: Output { input_chaining_value : [42 ; 8] , block : [42 ; 64] , counter : 42 , block_len : 42 , flags : 42 , platform : crate :: Platform :: Portable , } , position_within_block : 42 , } ; output_reader . zeroize () ; assert_eq ! (output_reader . inner . input_chaining_value , [0 ; 8]) ; assert_eq ! (output_reader . inner . block , [0 ; 64]) ; assert_eq ! (output_reader . inner . counter , 0) ; assert_eq ! (output_reader . inner . block_len , 0) ; assert_eq ! (output_reader . inner . flags , 0) ; assert ! (matches ! (output_reader . inner . platform , crate :: Platform :: Portable)) ; assert_eq ! (output_reader . position_within_block , 0) ; }
    };
}

test_zeroize!()