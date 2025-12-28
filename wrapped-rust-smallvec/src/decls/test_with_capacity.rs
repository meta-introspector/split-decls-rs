macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_with_capacity {
    () => {
        deps!();
        # [test] fn test_with_capacity () { let v : SmallVec < u8 , 3 > = SmallVec :: with_capacity (1) ; assert ! (v . is_empty ()) ; assert ! (! v . spilled ()) ; assert_eq ! (v . capacity () , 3) ; let v : SmallVec < u8 , 3 > = SmallVec :: with_capacity (10) ; assert ! (v . is_empty ()) ; assert ! (v . spilled ()) ; assert_eq ! (v . capacity () , 10) ; }
    };
}

test_with_capacity!();