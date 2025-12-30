// Generated macro for tests (module)
macro_rules! Depcrate_common_content_locationtests {
() => {
// Module: crate::common::content_location
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: test_decode ; use super :: * ; # [test] fn absolute_uri () { let s = "http://www.example.net/index.html" ; let loc = test_decode :: < ContentLocation > (& [s]) . unwrap () ; assert_eq ! (loc , ContentLocation (HeaderValue :: from_static (s))) ; } # [test] fn relative_uri_with_fragment () { let s = "/People.html#tim" ; let loc = test_decode :: < ContentLocation > (& [s]) . unwrap () ; assert_eq ! (loc , ContentLocation (HeaderValue :: from_static (s))) ; } }
};
}
