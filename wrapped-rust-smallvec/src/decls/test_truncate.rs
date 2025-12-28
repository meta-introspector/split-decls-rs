macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_truncate {
    () => {
        deps!();
        # [test] fn test_truncate () { let mut v : SmallVec < Box < u8 > , 8 > = SmallVec :: new () ; for x in 0 .. 8 { v . push (Box :: new (x)) ; } v . truncate (4) ; assert_eq ! (v . len () , 4) ; assert ! (! v . spilled ()) ; assert_eq ! (* v . swap_remove (1) , 1) ; assert_eq ! (* v . remove (1) , 3) ; v . insert (1 , Box :: new (3)) ; assert_eq ! (& v . iter () . map (| v | ** v) . collect ::< Vec < _ >> () , & [0 , 3 , 2]) ; }
    };
}

test_truncate!();