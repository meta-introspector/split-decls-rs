// Generated macro for test_set_extension (function)
macro_rules! Depcrate_teststest_set_extension {
() => {
// Module: crate::tests
// Provides: {"test_set_extension"}
// Dependencies: {}
# [test] pub fn test_set_extension () { macro_rules ! tse (($ path : expr , $ ext : expr , $ expected : expr , $ output : expr) => ({ let mut p = RelativePathBuf :: from ($ path) ; let output = p . set_extension ($ ext) ; assert ! (p . as_str () == $ expected && output == $ output , "setting extension of {:?} to {:?}: Expected {:?}/{:?}, got {:?}/{:?}" , $ path , $ ext , $ expected , $ output , p . as_str () , output) ; }) ;) ; tse ! ("foo" , "txt" , "foo.txt" , true) ; tse ! ("foo.bar" , "txt" , "foo.txt" , true) ; tse ! ("foo.bar.baz" , "txt" , "foo.bar.txt" , true) ; tse ! (".test" , "txt" , ".test.txt" , true) ; tse ! ("foo.txt" , "" , "foo" , true) ; tse ! ("foo" , "" , "foo" , true) ; tse ! ("" , "foo" , "" , false) ; tse ! ("." , "foo" , "." , false) ; tse ! ("foo/" , "bar" , "foo.bar" , true) ; tse ! ("foo/." , "bar" , "foo.bar" , true) ; tse ! (".." , "foo" , ".." , false) ; tse ! ("foo/.." , "bar" , "foo/.." , false) ; tse ! ("/" , "foo" , "/" , false) ; }
};
}
