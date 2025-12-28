macro_rules! deps {
    () => {
        DiagnosticsConfig!();
    };
}

macro_rules! minicore_smoke_test {
    () => {
        deps!();
        # [test] fn minicore_smoke_test () { if test_utils :: skip_slow_tests () { return ; } fn check (minicore : MiniCore) { let source = minicore . source_code (MiniCore :: RAW_SOURCE) ; let mut config = DiagnosticsConfig :: test_sample () ; config . disabled . insert ("unused_braces" . to_owned ()) ; config . disabled . insert ("unused_variables" . to_owned ()) ; config . disabled . insert ("remove-unnecessary-else" . to_owned ()) ; check_diagnostics_with_config (config , & source) ; } for flag in MiniCore :: available_flags (MiniCore :: RAW_SOURCE) { if flag == "clone" { continue ; } eprintln ! ("Checking minicore flag {flag}") ; check (MiniCore :: from_flags ([flag])) ; } eprintln ! ("Checking all minicore flags") ; check (MiniCore :: from_flags (MiniCore :: available_flags (MiniCore :: RAW_SOURCE))) }
    };
}

minicore_smoke_test!()