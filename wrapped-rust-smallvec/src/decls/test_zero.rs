macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_zero {
    () => {
        deps!();
        # [test] pub fn test_zero () { let mut v = SmallVec :: < _ , 0 > :: new () ; assert ! (! v . spilled ()) ; v . push (0usize) ; assert ! (v . spilled ()) ; assert_eq ! (&* v , & [0]) ; }
    };
}

test_zero!();