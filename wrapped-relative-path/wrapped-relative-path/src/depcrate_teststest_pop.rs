// Generated macro for test_pop (function)
macro_rules! Depcrate_teststest_pop {
() => {
// Module: crate::tests
// Provides: {"test_pop"}
// Dependencies: {}
# [test] pub fn test_pop () { macro_rules ! tp (($ path : expr , $ expected : expr , $ output : expr) => ({ let mut actual = RelativePathBuf :: from ($ path) ; let output = actual . pop () ; assert ! (actual . as_str () == $ expected && output == $ output , "popping from {:?}: Expected {:?}/{:?}, got {:?}/{:?}" , $ path , $ expected , $ output , actual . as_str () , output) ; }) ;) ; tp ! ("" , "" , false) ; tp ! ("/" , "" , true) ; tp ! ("foo" , "" , true) ; tp ! ("." , "" , true) ; tp ! ("/foo" , "" , true) ; tp ! ("/foo/bar" , "/foo" , true) ; tp ! ("/foo/bar/." , "/foo" , true) ; tp ! ("foo/bar" , "foo" , true) ; tp ! ("foo/." , "" , true) ; tp ! ("foo//bar" , "foo" , true) ; }
};
}
