macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_xof_partial_blocks {
    () => {
        deps!();
        # [test] fn test_xof_partial_blocks () { const OUT_LEN : usize = 6 * BLOCK_LEN ; let mut reference_out = [0u8 ; OUT_LEN] ; reference_impl :: Hasher :: new () . finalize (& mut reference_out) ; let mut all_at_once_out = [0u8 ; OUT_LEN] ; crate :: Hasher :: new () . finalize_xof () . fill (& mut all_at_once_out) ; assert_eq ! (reference_out , all_at_once_out) ; let mut partial_out = [0u8 ; OUT_LEN] ; let partial_start = 32 ; let partial_end = OUT_LEN - 32 ; let mut xof = crate :: Hasher :: new () . finalize_xof () ; xof . fill (& mut partial_out [.. partial_start]) ; xof . fill (& mut partial_out [partial_start .. partial_end]) ; xof . fill (& mut partial_out [partial_end ..]) ; assert_eq ! (reference_out , partial_out) ; }
    };
}

test_xof_partial_blocks!()