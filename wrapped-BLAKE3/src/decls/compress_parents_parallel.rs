macro_rules! deps {
    () => {
        IncrementCounter!();
        CVWords!();
    };
}

macro_rules! compress_parents_parallel {
    () => {
        deps!();
        fn compress_parents_parallel (child_chaining_values : & [u8] , key : & CVWords , flags : u8 , platform : Platform , out : & mut [u8] ,) -> usize { debug_assert_eq ! (child_chaining_values . len () % OUT_LEN , 0 , "wacky hash bytes") ; let num_children = child_chaining_values . len () / OUT_LEN ; debug_assert ! (num_children >= 2 , "not enough children") ; debug_assert ! (num_children <= 2 * MAX_SIMD_DEGREE_OR_2 , "too many") ; let mut parents_exact = child_chaining_values . chunks_exact (BLOCK_LEN) ; let mut parents_array = ArrayVec :: < & [u8 ; BLOCK_LEN] , MAX_SIMD_DEGREE_OR_2 > :: new () ; for parent in & mut parents_exact { parents_array . push (array_ref ! (parent , 0 , BLOCK_LEN)) ; } platform . hash_many (& parents_array , key , 0 , IncrementCounter :: No , flags | PARENT , 0 , 0 , out ,) ; let parents_so_far = parents_array . len () ; if ! parents_exact . remainder () . is_empty () { out [parents_so_far * OUT_LEN ..] [.. OUT_LEN] . copy_from_slice (parents_exact . remainder ()) ; parents_so_far + 1 } else { parents_so_far } }
    };
}

compress_parents_parallel!()