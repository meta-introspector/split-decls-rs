macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_exact_size_iterator {
    () => {
        deps!();
        # [test] fn test_exact_size_iterator () { let mut vec = SmallVec :: < u32 , 2 > :: from (& [1 , 2 , 3] [..]) ; assert_eq ! (vec . clone () . into_iter () . len () , 3) ; assert_eq ! (vec . drain (.. 2) . len () , 2) ; assert_eq ! (vec . into_iter () . len () , 1) ; }
    };
}

test_exact_size_iterator!()