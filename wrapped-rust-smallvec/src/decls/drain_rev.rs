macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! drain_rev {
    () => {
        deps!();
        # [test] fn drain_rev () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; assert_eq ! (v . drain (..) . rev () . collect ::< Vec < _ >> () , & [3]) ; v . push (3) ; v . push (4) ; v . push (5) ; assert_eq ! (v . drain (..) . rev () . collect ::< Vec < _ >> () , & [5 , 4 , 3]) ; }
    };
}

drain_rev!()