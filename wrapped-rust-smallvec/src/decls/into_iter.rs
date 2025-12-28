macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! into_iter {
    () => {
        deps!();
        # [test] fn into_iter () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; assert_eq ! (v . into_iter () . collect ::< Vec < _ >> () , & [3]) ; let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; v . push (4) ; v . push (5) ; assert_eq ! (v . into_iter () . collect ::< Vec < _ >> () , & [3 , 4 , 5]) ; }
    };
}

into_iter!()