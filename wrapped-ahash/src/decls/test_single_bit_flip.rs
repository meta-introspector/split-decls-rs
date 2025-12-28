macro_rules! test_single_bit_flip {
    () => {
        fn test_single_bit_flip < T : Hasher > (hasher : impl Fn () -> T) { let size = 32 ; let compare_value = hash (& 0u32 , & hasher) ; for pos in 0 .. size { let test_value = hash (& (1u32 << pos) , & hasher) ; assert_sufficiently_different (compare_value , test_value , 2) ; } let size = 64 ; let compare_value = hash (& 0u64 , & hasher) ; for pos in 0 .. size { let test_value = hash (& (1u64 << pos) , & hasher) ; assert_sufficiently_different (compare_value , test_value , 2) ; } let size = 128 ; let compare_value = hash (& 0u128 , & hasher) ; for pos in 0 .. size { let test_value = hash (& (1u128 << pos) , & hasher) ; dbg ! (compare_value , test_value) ; assert_sufficiently_different (compare_value , test_value , 2) ; } }
    };
}

test_single_bit_flip!()