macro_rules! deps {
    () => {
        NaiveTime!();
    };
}

macro_rules! test_time_hms {
    () => {
        deps!();
        # [test] fn test_time_hms () { assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . hour () , 3) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_hour (0) , Some (NaiveTime :: from_hms_opt (0 , 5 , 7) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_hour (23) , Some (NaiveTime :: from_hms_opt (23 , 5 , 7) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_hour (24) , None) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_hour (u32 :: MAX) , None) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . minute () , 5) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_minute (0) , Some (NaiveTime :: from_hms_opt (3 , 0 , 7) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_minute (59) , Some (NaiveTime :: from_hms_opt (3 , 59 , 7) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_minute (60) , None) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_minute (u32 :: MAX) , None) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . second () , 7) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_second (0) , Some (NaiveTime :: from_hms_opt (3 , 5 , 0) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_second (59) , Some (NaiveTime :: from_hms_opt (3 , 5 , 59) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_second (60) , None) ; assert_eq ! (NaiveTime :: from_hms_opt (3 , 5 , 7) . unwrap () . with_second (u32 :: MAX) , None) ; }
    };
}

test_time_hms!();