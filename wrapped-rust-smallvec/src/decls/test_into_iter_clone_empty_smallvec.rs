macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_into_iter_clone_empty_smallvec {
    () => {
        deps!();
        # [test] fn test_into_iter_clone_empty_smallvec () { let mut iter = SmallVec :: < u8 , 2 > :: new () . into_iter () ; let mut clone_iter = iter . clone () ; assert_eq ! (iter . next () , None) ; assert_eq ! (clone_iter . next () , None) ; }
    };
}

test_into_iter_clone_empty_smallvec!()