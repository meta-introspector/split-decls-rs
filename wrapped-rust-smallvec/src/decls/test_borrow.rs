macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_borrow {
    () => {
        deps!();
        # [test] fn test_borrow () { use std :: borrow :: Borrow ; let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; a . push (1) ; assert_eq ! (a . borrow () , [1]) ; a . push (2) ; assert_eq ! (a . borrow () , [1 , 2]) ; a . push (3) ; assert_eq ! (a . borrow () , [1 , 2 , 3]) ; }
    };
}

test_borrow!();