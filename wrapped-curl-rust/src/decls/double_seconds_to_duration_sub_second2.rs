macro_rules! double_seconds_to_duration_sub_second2 {
    () => {
        # [test] fn double_seconds_to_duration_sub_second2 () { let dur = double_seconds_to_duration (0.5) ; assert_eq ! (dur . as_secs () , 0) ; assert_eq ! (dur . subsec_nanos () , 500_000_000) ; }
    };
}

double_seconds_to_duration_sub_second2!()