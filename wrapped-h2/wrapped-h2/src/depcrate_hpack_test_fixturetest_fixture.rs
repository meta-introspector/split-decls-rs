// Generated macro for test_fixture (function)
macro_rules! Depcrate_hpack_test_fixturetest_fixture {
() => {
// Module: crate::hpack::test::fixture
// Provides: {"test_fixture"}
// Dependencies: {}
fn test_fixture (path : & Path) { let mut file = File :: open (path) . unwrap () ; let mut data = String :: new () ; file . read_to_string (& mut data) . unwrap () ; let story : Value = serde_json :: from_str (& data) . unwrap () ; test_story (story) ; }
};
}
