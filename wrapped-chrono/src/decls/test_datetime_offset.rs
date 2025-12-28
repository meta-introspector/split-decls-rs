macro_rules! deps {
    () => {
        TimeDelta!();
        FixedOffset!();
        Utc!();
    };
}

macro_rules! test_datetime_offset {
    () => {
        deps!();
        # [test] fn test_datetime_offset () { let est = FixedOffset :: west_opt (5 * 60 * 60) . unwrap () ; let edt = FixedOffset :: west_opt (4 * 60 * 60) . unwrap () ; let kst = FixedOffset :: east_opt (9 * 60 * 60) . unwrap () ; assert_eq ! (format ! ("{}" , Utc . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap ()) , "2014-05-06 07:08:09 UTC") ; assert_eq ! (format ! ("{}" , edt . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap ()) , "2014-05-06 07:08:09 -04:00") ; assert_eq ! (format ! ("{}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap ()) , "2014-05-06 07:08:09 +09:00") ; assert_eq ! (format ! ("{:?}" , Utc . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap ()) , "2014-05-06T07:08:09Z") ; assert_eq ! (format ! ("{:?}" , edt . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap ()) , "2014-05-06T07:08:09-04:00") ; assert_eq ! (format ! ("{:?}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap ()) , "2014-05-06T07:08:09+09:00") ; assert_eq ! (format ! ("{:?}" , Utc . with_ymd_and_hms (2014 , 5 , 6 , 0 , 0 , 0) . unwrap ()) , "2014-05-06T00:00:00Z") ; assert_eq ! (format ! ("{:?}" , edt . with_ymd_and_hms (2014 , 5 , 6 , 0 , 0 , 0) . unwrap ()) , "2014-05-06T00:00:00-04:00") ; assert_eq ! (format ! ("{:?}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 0 , 0 , 0) . unwrap ()) , "2014-05-06T00:00:00+09:00") ; assert_eq ! (format ! ("{:?}" , Utc . with_ymd_and_hms (2014 , 5 , 6 , 23 , 59 , 59) . unwrap ()) , "2014-05-06T23:59:59Z") ; assert_eq ! (format ! ("{:?}" , edt . with_ymd_and_hms (2014 , 5 , 6 , 23 , 59 , 59) . unwrap ()) , "2014-05-06T23:59:59-04:00") ; assert_eq ! (format ! ("{:?}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 23 , 59 , 59) . unwrap ()) , "2014-05-06T23:59:59+09:00") ; let dt = Utc . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () ; assert_eq ! (dt , edt . with_ymd_and_hms (2014 , 5 , 6 , 3 , 8 , 9) . unwrap ()) ; assert_eq ! (dt + TimeDelta :: try_seconds (3600 + 60 + 1) . unwrap () , Utc . with_ymd_and_hms (2014 , 5 , 6 , 8 , 9 , 10) . unwrap ()) ; assert_eq ! (dt . signed_duration_since (edt . with_ymd_and_hms (2014 , 5 , 6 , 10 , 11 , 12) . unwrap ()) , TimeDelta :: try_seconds (- 7 * 3600 - 3 * 60 - 3) . unwrap ()) ; assert_eq ! (* Utc . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () . offset () , Utc) ; assert_eq ! (* edt . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () . offset () , edt) ; assert ! (* edt . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () . offset () != est) ; }
    };
}

test_datetime_offset!()