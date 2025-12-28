macro_rules! deps {
    () => {
        DiagnosticsConfig!();
    };
}

macro_rules! check_nth_fix {
    () => {
        deps!();
        # [track_caller] fn check_nth_fix (nth : usize , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let mut config = DiagnosticsConfig :: test_sample () ; config . expr_fill_default = ExprFillDefaultMode :: Default ; check_nth_fix_with_config (config , nth , ra_fixture_before , ra_fixture_after) }
    };
}

check_nth_fix!()