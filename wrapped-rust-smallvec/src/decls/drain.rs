macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! drain {
    () => {
        deps!();
        # [test] fn drain () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; assert_eq ! (v . drain (..) . collect ::< Vec < _ >> () , & [3]) ; v . push (3) ; v . push (4) ; v . push (5) ; let old_capacity = v . capacity () ; assert_eq ! (v . drain (1 ..) . collect ::< Vec < _ >> () , & [4 , 5]) ; assert_eq ! (v . capacity () , old_capacity) ; let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (1) ; v . push (2) ; assert_eq ! (v . drain (.. 1) . collect ::< Vec < _ >> () , & [1]) ; }
    };
}

drain!()