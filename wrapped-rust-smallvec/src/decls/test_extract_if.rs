macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_extract_if {
    () => {
        deps!();
        # [cfg (feature = "extract_if")] # [test] fn test_extract_if () { let mut a : SmallVec < u8 , 2 > = smallvec ! [0 , 1u8 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 0] ; let b : SmallVec < u8 , 2 > = a . extract_if (1 .. 9 , | x | * x % 3 == 0) . collect () ; assert_eq ! (a , SmallVec ::< u8 , 2 >:: from (& [0 , 1u8 , 2 , 4 , 5 , 7 , 8 , 0])) ; assert_eq ! (b , SmallVec ::< u8 , 2 >:: from (& [3u8 , 6])) ; }
    };
}

test_extract_if!();