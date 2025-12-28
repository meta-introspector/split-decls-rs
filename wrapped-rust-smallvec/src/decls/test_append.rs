macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_append {
    () => {
        deps!();
        # [test] fn test_append () { let mut v : SmallVec < u8 , 8 > = SmallVec :: new () ; for x in 0 .. 4 { v . push (x) ; } assert_eq ! (v . len () , 4) ; let mut n : SmallVec < u8 , 2 > = SmallVec :: from_buf ([5 , 6]) ; v . append (& mut n) ; assert_eq ! (v . len () , 6) ; assert_eq ! (n . len () , 0) ; assert_eq ! (& v . iter () . map (| v | * v) . collect ::< Vec < _ >> () , & [0 , 1 , 2 , 3 , 5 , 6]) ; }
    };
}

test_append!()