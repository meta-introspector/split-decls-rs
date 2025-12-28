macro_rules! deps {
    () => {
        DiagnosticsConfig!();
    };
}

macro_rules! check_diagnostics {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_diagnostics (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { let mut config = DiagnosticsConfig :: test_sample () ; config . disabled . insert ("inactive-code" . to_owned ()) ; check_diagnostics_with_config (config , ra_fixture) }
    };
}

check_diagnostics!();