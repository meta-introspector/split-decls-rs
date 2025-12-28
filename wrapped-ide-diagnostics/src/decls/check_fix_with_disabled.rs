macro_rules! deps {
    () => {
        DiagnosticsConfig!();
    };
}

macro_rules! check_fix_with_disabled {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_fix_with_disabled (# [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str , disabled : impl Iterator < Item = String > ,) { let mut config = DiagnosticsConfig :: test_sample () ; config . expr_fill_default = ExprFillDefaultMode :: Default ; config . disabled . extend (disabled) ; check_nth_fix_with_config (config , 0 , ra_fixture_before , ra_fixture_after) }
    };
}

check_fix_with_disabled!();