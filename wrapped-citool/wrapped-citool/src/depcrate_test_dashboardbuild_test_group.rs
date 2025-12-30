// Generated macro for build_test_group (function)
macro_rules! Depcrate_test_dashboardbuild_test_group {
() => {
// Module: crate::test_dashboard
// Provides: {"build_test_group"}
// Dependencies: {}
# [doc = " Recursively expand a test group based on filesystem hierarchy."] fn build_test_group < 'a > (name : & str , tests : BTreeMap < String , Test < 'a > >) -> TestGroup < 'a > { let mut root_tests = vec ! [] ; let mut subdirs : BTreeMap < String , BTreeMap < String , Test < 'a > > > = Default :: default () ; for (name , test) in tests { let mut components = Path :: new (& name) . components () . peekable () ; let subdir = components . next () . unwrap () ; if components . peek () . is_none () { root_tests . push ((name , test)) ; } else { let subdir_tests = subdirs . entry (subdir . as_os_str () . to_str () . unwrap () . to_string ()) . or_default () ; let test_name = components . into_iter () . collect :: < PathBuf > () . to_str () . unwrap () . to_string () ; subdir_tests . insert (test_name , test) ; } } let dirs = subdirs . into_iter () . map (| (name , tests) | { let group = build_test_group (& name , tests) ; (name , group) }) . collect () ; TestGroup { name : name . to_string () , root_tests , groups : dirs } }
};
}
