macro_rules! double_seconds_to_duration_sub_second1 {
    () => {
        # [test] fn double_seconds_to_duration_sub_second1 () { let dur = double_seconds_to_duration (0.0) ; assert_eq ! (dur . as_secs () , 0) ; assert_eq ! (dur . subsec_nanos () , 0) ; }
    };
}

double_seconds_to_duration_sub_second1!();