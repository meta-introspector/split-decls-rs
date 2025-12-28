macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! grow_spilled_same_size {
    () => {
        deps!();
        # [test] fn grow_spilled_same_size () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (0) ; v . push (1) ; v . push (2) ; assert ! (v . spilled ()) ; assert_eq ! (v . capacity () , 4) ; v . grow (4) ; assert_eq ! (v . capacity () , 4) ; assert_eq ! (v [..] , [0 , 1 , 2]) ; }
    };
}

grow_spilled_same_size!();