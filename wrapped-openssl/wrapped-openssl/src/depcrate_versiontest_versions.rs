// Generated macro for test_versions (function)
macro_rules! Depcrate_versiontest_versions {
() => {
// Module: crate::version
// Provides: {"test_versions"}
// Dependencies: {}
# [doc = " This test ensures that we do not segfault when calling the functions of this module"] # [doc = " and that the strings respect a reasonable format."] # [test] fn test_versions () { println ! ("Number: '{}'" , number ()) ; println ! ("Version: '{}'" , version ()) ; println ! ("C flags: '{}'" , c_flags ()) ; println ! ("Built on: '{}'" , built_on ()) ; println ! ("Platform: '{}'" , platform ()) ; println ! ("Dir: '{}'" , dir ()) ; # [cfg (not (any (libressl , boringssl , awslc)))] fn expected_name () -> & 'static str { "OpenSSL" } # [cfg (libressl)] fn expected_name () -> & 'static str { "LibreSSL" } # [cfg (boringssl)] fn expected_name () -> & 'static str { "BoringSSL" } # [cfg (awslc)] fn expected_name () -> & 'static str { "AWS-LC" } assert ! (number () > 0) ; assert ! (version () . starts_with (expected_name ())) ; assert ! (c_flags () . starts_with ("compiler:")) ; if ! built_on () . is_empty () { assert ! (built_on () . starts_with ("built on:")) ; } }
};
}
