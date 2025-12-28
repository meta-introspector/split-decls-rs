macro_rules! double_seconds_to_duration_whole_second {
    () => {
        # [test] fn double_seconds_to_duration_whole_second () { let dur = double_seconds_to_duration (1.0) ; assert_eq ! (dur . as_secs () , 1) ; assert_eq ! (dur . subsec_nanos () , 0) ; }
    };
}

double_seconds_to_duration_whole_second!();