macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_truncate_references {
    () => {
        deps!();
        # [test] fn test_truncate_references () { let mut v = vec ! [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] ; let mut i = 8 ; let mut v : SmallVec < & mut u8 , 8 > = v . iter_mut () . collect () ; v . truncate (4) ; assert_eq ! (v . len () , 4) ; assert ! (! v . spilled ()) ; assert_eq ! (* v . swap_remove (1) , 1) ; assert_eq ! (* v . remove (1) , 3) ; v . insert (1 , & mut i) ; assert_eq ! (& v . iter_mut () . map (| v | & mut ** v) . collect ::< Vec < _ >> () , & [& mut 0 , & mut 8 , & mut 2]) ; }
    };
}

test_truncate_references!();