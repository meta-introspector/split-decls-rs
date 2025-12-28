macro_rules! test_all_bytes_matter {
    () => {
        fn test_all_bytes_matter < T : Hasher > (hasher : impl Fn () -> T) { let mut item = vec ! [0 ; 256] ; let base_hash = hash (& item , & hasher) ; for pos in 0 .. 256 { item [pos] = 255 ; let hash = hash (& item , & hasher) ; assert_ne ! (base_hash , hash , "Position {} did not affect output" , pos) ; item [pos] = 0 ; } }
    };
}

test_all_bytes_matter!();