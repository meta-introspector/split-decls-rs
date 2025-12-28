macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_borrow_mut {
    () => {
        deps!();
        # [test] fn test_borrow_mut () { use std :: borrow :: BorrowMut ; let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; a . push (1) ; assert_eq ! (a . borrow_mut () , [1]) ; a . push (2) ; assert_eq ! (a . borrow_mut () , [1 , 2]) ; a . push (3) ; assert_eq ! (a . borrow_mut () , [1 , 2 , 3]) ; BorrowMut :: < [u32] > :: borrow_mut (& mut a) [1] = 4 ; assert_eq ! (a . borrow_mut () , [1 , 4 , 3]) ; }
    };
}

test_borrow_mut!()