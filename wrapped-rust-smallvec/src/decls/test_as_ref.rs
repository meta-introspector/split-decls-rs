macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_as_ref {
    () => {
        deps!();
        # [test] fn test_as_ref () { let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; a . push (1) ; assert_eq ! (a . as_ref () , [1]) ; a . push (2) ; assert_eq ! (a . as_ref () , [1 , 2]) ; a . push (3) ; assert_eq ! (a . as_ref () , [1 , 2 , 3]) ; }
    };
}

test_as_ref!();