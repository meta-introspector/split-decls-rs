macro_rules! recording_match_fail_reasons {
    () => {
        fn recording_match_fail_reasons () -> bool { RECORDING_MATCH_FAIL_REASONS . with (| c | c . get ()) }
    };
}

recording_match_fail_reasons!()