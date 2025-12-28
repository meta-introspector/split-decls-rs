macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_as_mut {
    () => {
        deps!();
        # [test] fn test_as_mut () { let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; a . push (1) ; assert_eq ! (a . as_mut () , [1]) ; a . push (2) ; assert_eq ! (a . as_mut () , [1 , 2]) ; a . push (3) ; assert_eq ! (a . as_mut () , [1 , 2 , 3]) ; a . as_mut () [1] = 4 ; assert_eq ! (a . as_mut () , [1 , 4 , 3]) ; }
    };
}

test_as_mut!();