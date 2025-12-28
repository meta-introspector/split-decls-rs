macro_rules! deps {
    () => {
        FixedOffset!();
        NaiveDate!();
        SecondsFormat!();
    };
}

macro_rules! test_rfc3339_opts {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn test_rfc3339_opts () { use crate :: SecondsFormat :: * ; let pst = FixedOffset :: east_opt (8 * 60 * 60) . unwrap () ; let dt = pst . from_local_datetime (& NaiveDate :: from_ymd_opt (2018 , 1 , 11) . unwrap () . and_hms_nano_opt (10 , 5 , 13 , 84_660_000) . unwrap () ,) . unwrap () ; assert_eq ! (dt . to_rfc3339_opts (Secs , false) , "2018-01-11T10:05:13+08:00") ; assert_eq ! (dt . to_rfc3339_opts (Secs , true) , "2018-01-11T10:05:13+08:00") ; assert_eq ! (dt . to_rfc3339_opts (Millis , false) , "2018-01-11T10:05:13.084+08:00") ; assert_eq ! (dt . to_rfc3339_opts (Micros , false) , "2018-01-11T10:05:13.084660+08:00") ; assert_eq ! (dt . to_rfc3339_opts (Nanos , false) , "2018-01-11T10:05:13.084660000+08:00") ; assert_eq ! (dt . to_rfc3339_opts (AutoSi , false) , "2018-01-11T10:05:13.084660+08:00") ; let ut = dt . naive_utc () . and_utc () ; assert_eq ! (ut . to_rfc3339_opts (Secs , false) , "2018-01-11T02:05:13+00:00") ; assert_eq ! (ut . to_rfc3339_opts (Secs , true) , "2018-01-11T02:05:13Z") ; assert_eq ! (ut . to_rfc3339_opts (Millis , false) , "2018-01-11T02:05:13.084+00:00") ; assert_eq ! (ut . to_rfc3339_opts (Millis , true) , "2018-01-11T02:05:13.084Z") ; assert_eq ! (ut . to_rfc3339_opts (Micros , true) , "2018-01-11T02:05:13.084660Z") ; assert_eq ! (ut . to_rfc3339_opts (Nanos , true) , "2018-01-11T02:05:13.084660000Z") ; assert_eq ! (ut . to_rfc3339_opts (AutoSi , true) , "2018-01-11T02:05:13.084660Z") ; }
    };
}

test_rfc3339_opts!()