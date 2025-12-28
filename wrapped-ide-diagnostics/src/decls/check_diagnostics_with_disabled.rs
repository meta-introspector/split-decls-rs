macro_rules! deps {
    () => {
        DiagnosticsConfig!();
    };
}

macro_rules! check_diagnostics_with_disabled {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_diagnostics_with_disabled (# [rust_analyzer :: rust_fixture] ra_fixture : & str , disabled : & [& str] ,) { let mut config = DiagnosticsConfig :: test_sample () ; config . disabled . extend (disabled . iter () . map (| & s | s . to_owned ())) ; check_diagnostics_with_config (config , ra_fixture) }
    };
}

check_diagnostics_with_disabled!();