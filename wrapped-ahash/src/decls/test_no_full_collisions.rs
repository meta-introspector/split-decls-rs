macro_rules! test_no_full_collisions {
    () => {
        fn test_no_full_collisions < T : Hasher > (gen_hash : impl Fn () -> T) { let options : [u32 ; 11] = [0x00000000 , 0x10000000 , 0x20000000 , 0x40000000 , 0x80000000 , 0xF0000000 , 1 , 2 , 4 , 8 , 15 ,] ; let mut combinations = Vec :: new () ; gen_combinations (& options , 7 , Vec :: new () , & mut combinations) ; let mut map : HashMap < u64 , Vec < u8 > > = HashMap :: new () ; for combination in combinations { use zerocopy :: IntoBytes ; let array = combination . as_bytes () . to_vec () ; let mut hasher = gen_hash () ; hasher . write (& array) ; let hash = hasher . finish () ; if let Some (value) = map . get (& hash) { assert_eq ! (value , & array , "Found a collision between {:x?} and {:x?}. Hash: {:x?}" , value , & array , & hash) ; } else { map . insert (hash , array) ; } } assert_eq ! (21435887 , map . len ()) ; }
    };
}

test_no_full_collisions!();