macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_into_iter_clone {
    () => {
        deps!();
        # [test] fn test_into_iter_clone () { let mut iter = SmallVec :: < u8 , 2 > :: from_iter (0 .. 3) . into_iter () ; let mut clone_iter = iter . clone () ; while let Some (x) = iter . next () { assert_eq ! (x , clone_iter . next () . unwrap ()) ; } assert_eq ! (clone_iter . next () , None) ; }
    };
}

test_into_iter_clone!();