macro_rules! test_sparse {
    () => {
        fn test_sparse < T : Hasher > (hasher : impl Fn () -> T) { use smallvec :: SmallVec ; let mut buf = [0u8 ; 256] ; let mut hashes = HashMap :: new () ; for idx_1 in 0 .. 255_u8 { for idx_2 in idx_1 + 1 ..= 255_u8 { for value_1 in [1 , 2 , 4 , 8 , 16 , 32 , 64 , 128] { for value_2 in [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 12 , 15 , 16 , 17 , 18 , 20 , 24 , 31 , 32 , 33 , 48 , 64 , 96 , 127 , 128 , 129 , 192 , 254 , 255 ,] { buf [idx_1 as usize] = value_1 ; buf [idx_2 as usize] = value_2 ; let hash_value = hash_with (& buf , & mut hasher ()) ; let keys = hashes . entry (hash_value) . or_insert (SmallVec :: < [[u8 ; 4] ; 1] > :: new ()) ; keys . push ([idx_1 , value_1 , idx_2 , value_2]) ; buf [idx_1 as usize] = 0 ; buf [idx_2 as usize] = 0 ; } } } } hashes . retain (| _key , value | value . len () != 1) ; assert_eq ! (0 , hashes . len () , "Collision with: {:?}" , hashes) ; }
    };
}

test_sparse!()