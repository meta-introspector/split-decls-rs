macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! into_iter_rev {
    () => {
        deps!();
        # [test] fn into_iter_rev () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; assert_eq ! (v . into_iter () . rev () . collect ::< Vec < _ >> () , & [3]) ; let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; v . push (4) ; v . push (5) ; assert_eq ! (v . into_iter () . rev () . collect ::< Vec < _ >> () , & [5 , 4 , 3]) ; }
    };
}

into_iter_rev!();