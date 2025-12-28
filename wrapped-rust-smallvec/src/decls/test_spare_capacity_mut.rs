macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_spare_capacity_mut {
    () => {
        deps!();
        # [test] fn test_spare_capacity_mut () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; assert ! (! v . spilled ()) ; let spare = v . spare_capacity_mut () ; assert_eq ! (spare . len () , 2) ; assert_eq ! (spare . as_ptr () . cast ::< u8 > () , v . as_ptr ()) ; v . push (1) ; assert ! (! v . spilled ()) ; let spare = v . spare_capacity_mut () ; assert_eq ! (spare . len () , 1) ; assert_eq ! (spare . as_ptr () . cast ::< u8 > () , unsafe { v . as_ptr () . add (1) }) ; v . push (2) ; assert ! (! v . spilled ()) ; let spare = v . spare_capacity_mut () ; assert_eq ! (spare . len () , 0) ; assert_eq ! (spare . as_ptr () . cast ::< u8 > () , unsafe { v . as_ptr () . add (2) }) ; v . push (3) ; assert ! (v . spilled ()) ; let spare = v . spare_capacity_mut () ; assert ! (spare . len () >= 1) ; assert_eq ! (spare . as_ptr () . cast ::< u8 > () , unsafe { v . as_ptr () . add (3) }) ; }
    };
}

test_spare_capacity_mut!();