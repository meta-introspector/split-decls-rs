macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_extend_from_slice {
    () => {
        deps!();
        # [test] fn test_extend_from_slice () { let mut v : SmallVec < u8 , 8 > = SmallVec :: new () ; for x in 0 .. 4 { v . push (x) ; } assert_eq ! (v . len () , 4) ; v . extend_from_slice (& [5 , 6]) ; assert_eq ! (& v . iter () . map (| v | * v) . collect ::< Vec < _ >> () , & [0 , 1 , 2 , 3 , 5 , 6]) ; }
    };
}

test_extend_from_slice!();