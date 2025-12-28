macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_ord {
    () => {
        deps!();
        # [test] fn test_ord () { let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; let mut b : SmallVec < u32 , 2 > = SmallVec :: new () ; let mut c : SmallVec < u32 , 2 > = SmallVec :: new () ; a . push (1) ; b . push (1) ; b . push (1) ; c . push (1) ; c . push (2) ; assert ! (a < b) ; assert ! (b > a) ; assert ! (b < c) ; assert ! (c > b) ; }
    };
}

test_ord!()