macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_split_off_take_all {
    () => {
        deps!();
        # [test] fn test_split_off_take_all () { let mut vec = SmallVec :: < u32 , 4 > :: with_capacity (1000) ; vec . extend ([1 , 2 , 3 , 4 , 5 , 6]) ; let orig_ptr = vec . as_ptr () ; let orig_capacity : usize = vec . capacity () ; let split_off = vec . split_off (0) ; assert_eq ! (& vec [..] , & []) ; assert_eq ! (& split_off [..] , & [1 , 2 , 3 , 4 , 5 , 6]) ; assert_eq ! (vec . capacity () , orig_capacity) ; assert_eq ! (vec . as_ptr () , orig_ptr) ; assert ! (split_off . capacity () < orig_capacity) ; assert_ne ! (split_off . as_ptr () , orig_ptr) ; }
    };
}

test_split_off_take_all!();