macro_rules! deps {
    () => {
        NaiveTime!();
        FixedOffset!();
    };
}

macro_rules! test_overflowing_offset {
    () => {
        deps!();
        # [test] fn test_overflowing_offset () { let hmsm = | h , m , s , n | NaiveTime :: from_hms_milli_opt (h , m , s , n) . unwrap () ; let positive_offset = FixedOffset :: east_opt (4 * 60 * 60) . unwrap () ; let t = hmsm (5 , 6 , 7 , 890) ; assert_eq ! (t . overflowing_add_offset (positive_offset) , (hmsm (9 , 6 , 7 , 890) , 0)) ; assert_eq ! (t . overflowing_sub_offset (positive_offset) , (hmsm (1 , 6 , 7 , 890) , 0)) ; let t = hmsm (23 , 59 , 59 , 1_000) ; assert_eq ! (t . overflowing_add_offset (positive_offset) , (hmsm (3 , 59 , 59 , 1_000) , 1)) ; assert_eq ! (t . overflowing_sub_offset (positive_offset) , (hmsm (19 , 59 , 59 , 1_000) , 0)) ; let t = hmsm (1 , 2 , 3 , 456) ; assert_eq ! (t . overflowing_sub_offset (positive_offset) , (hmsm (21 , 2 , 3 , 456) , - 1)) ; let negative_offset = FixedOffset :: west_opt (((2 * 60) + 3) * 60 + 4) . unwrap () ; let t = hmsm (5 , 6 , 7 , 890) ; assert_eq ! (t . overflowing_add_offset (negative_offset) , (hmsm (3 , 3 , 3 , 890) , 0)) ; assert_eq ! (t . overflowing_sub_offset (negative_offset) , (hmsm (7 , 9 , 11 , 890) , 0)) ; assert_eq ! (t . overflowing_add_offset (positive_offset) . 0 , t + positive_offset) ; assert_eq ! (t . overflowing_sub_offset (positive_offset) . 0 , t - positive_offset) ; }
    };
}

test_overflowing_offset!();