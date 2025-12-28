macro_rules! record_match_fails_reasons_scope {
    () => {
        pub (crate) fn record_match_fails_reasons_scope < F , T > (debug_active : bool , f : F) -> T where F : Fn () -> T , { RECORDING_MATCH_FAIL_REASONS . with (| c | c . set (debug_active)) ; let res = f () ; RECORDING_MATCH_FAIL_REASONS . with (| c | c . set (false)) ; res }
    };
}

record_match_fails_reasons_scope!()