// Generated macro for detect_src_and_out (function)
macro_rules! Depcrate_core_config_testsdetect_src_and_out {
() => {
// Module: crate::core::config::tests
// Provides: {"detect_src_and_out"}
// Dependencies: {}
# [test] fn detect_src_and_out () { fn test (cfg : Config , build_dir : Option < & str >) { let current_dir = std :: env :: current_dir () . unwrap () ; let expected_src = current_dir . ancestors () . nth (2) . unwrap () ; assert_eq ! (& cfg . src , expected_src) ; let manifest_dir = Path :: new (env ! ("CARGO_MANIFEST_DIR")) ; let expected_src = manifest_dir . ancestors () . nth (2) . unwrap () ; assert_eq ! (& cfg . src , expected_src) ; if let Some (custom_build_dir) = build_dir { assert_eq ! (& cfg . out , Path :: new (custom_build_dir)) ; } else { let cargo_target_dir = env :: var_os ("CARGO_TARGET_DIR") . expect ("CARGO_TARGET_DIR must been provided for the test environment from bootstrap" ,) ; let expected_out = Path :: new (& cargo_target_dir) . parent () . unwrap () ; assert_eq ! (& cfg . out , expected_out) ; let args : Vec < String > = env :: args () . collect () ; let dep = Path :: new (args . first () . unwrap ()) ; let expected_out = dep . ancestors () . nth (5) . unwrap () ; assert_eq ! (& cfg . out , expected_out) ; } } test (parse ("") , None) ; { let build_dir = if cfg ! (windows) { "C:\\tmp" } else { "/tmp" } ; test (parse (& format ! ("build.build-dir = '{build_dir}'")) , Some (build_dir)) ; } }
};
}
