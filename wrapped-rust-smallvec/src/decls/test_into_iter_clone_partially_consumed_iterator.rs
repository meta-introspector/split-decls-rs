macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_into_iter_clone_partially_consumed_iterator {
    () => {
        deps!();
        # [test] fn test_into_iter_clone_partially_consumed_iterator () { let mut iter = SmallVec :: < u8 , 2 > :: from_iter (0 .. 3) . into_iter () . skip (1) ; let mut clone_iter = iter . clone () ; while let Some (x) = iter . next () { assert_eq ! (x , clone_iter . next () . unwrap ()) ; } assert_eq ! (clone_iter . next () , None) ; }
    };
}

test_into_iter_clone_partially_consumed_iterator!()