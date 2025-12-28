macro_rules! test_split_aligned {
    () => {
        # [cfg (feature = "std")] # [test] fn test_split_aligned () { let data = vec ! [0 ; 1024] ; assert_eq ! (data . as_ptr () as usize & 7 , 0) ; let (a , b , c) = split_aligned_for :: < u8 > (& data) ; assert_eq ! (a . len () , 0) ; assert_eq ! (b . len () , data . len ()) ; assert_eq ! (c . len () , 0) ; let (a , b , c) = split_aligned_for :: < u64 > (& data) ; assert_eq ! (a . len () , 0) ; assert_eq ! (b . len () , data . len () / 8) ; assert_eq ! (c . len () , 0) ; let offset1 = & data [1 .. data . len () - 2] ; let (a , b , c) = split_aligned_for :: < u64 > (offset1) ; assert_eq ! (a . len () , 7) ; assert_eq ! (b . len () , data . len () / 8 - 2) ; assert_eq ! (c . len () , 6) ; let data = [0 ; 7] ; let (a , b , c) = split_aligned_for :: < u64 > (& data) ; assert_eq ! (a . len () + c . len () , 7) ; assert_eq ! (b . len () , 0) ; }
    };
}

test_split_aligned!();