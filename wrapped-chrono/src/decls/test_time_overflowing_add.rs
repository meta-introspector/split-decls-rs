macro_rules! deps {
    () => {
        NaiveTime!();
        TimeDelta!();
    };
}

macro_rules! test_time_overflowing_add {
    () => {
        deps!();
        # [test] fn test_time_overflowing_add () { let hmsm = | h , m , s , ms | NaiveTime :: from_hms_milli_opt (h , m , s , ms) . unwrap () ; assert_eq ! (hmsm (3 , 4 , 5 , 678) . overflowing_add_signed (TimeDelta :: try_hours (11) . unwrap ()) , (hmsm (14 , 4 , 5 , 678) , 0)) ; assert_eq ! (hmsm (3 , 4 , 5 , 678) . overflowing_add_signed (TimeDelta :: try_hours (23) . unwrap ()) , (hmsm (2 , 4 , 5 , 678) , 86_400)) ; assert_eq ! (hmsm (3 , 4 , 5 , 678) . overflowing_add_signed (TimeDelta :: try_hours (- 7) . unwrap ()) , (hmsm (20 , 4 , 5 , 678) , - 86_400)) ; assert_eq ! (hmsm (3 , 4 , 59 , 1_678) . overflowing_add_signed (TimeDelta :: try_days (1) . unwrap ()) , (hmsm (3 , 4 , 59 , 678) , 86_400)) ; assert_eq ! (hmsm (3 , 4 , 59 , 1_678) . overflowing_add_signed (TimeDelta :: try_days (- 1) . unwrap ()) , (hmsm (3 , 5 , 0 , 678) , - 86_400)) ; }
    };
}

test_time_overflowing_add!()