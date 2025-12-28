macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_capacity {
    () => {
        deps!();
        # [test] fn test_capacity () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . reserve (1) ; assert_eq ! (v . capacity () , 2) ; assert ! (! v . spilled ()) ; v . reserve_exact (0x100) ; assert ! (v . capacity () >= 0x100) ; v . push (0) ; v . push (1) ; v . push (2) ; v . push (3) ; v . shrink_to_fit () ; assert ! (v . capacity () < 0x100) ; }
    };
}

test_capacity!()