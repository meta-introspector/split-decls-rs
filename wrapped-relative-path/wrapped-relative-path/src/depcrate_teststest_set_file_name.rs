// Generated macro for test_set_file_name (function)
macro_rules! Depcrate_teststest_set_file_name {
() => {
// Module: crate::tests
// Provides: {"test_set_file_name"}
// Dependencies: {}
# [test] pub fn test_set_file_name () { macro_rules ! tfn (($ path : expr , $ file : expr , $ expected : expr) => ({ let mut p = RelativePathBuf :: from ($ path) ; p . set_file_name ($ file) ; assert ! (p . as_str () == $ expected , "setting file name of {:?} to {:?}: Expected {:?}, got {:?}" , $ path , $ file , $ expected , p . as_str ()) ; }) ;) ; tfn ! ("foo" , "foo" , "foo") ; tfn ! ("foo" , "bar" , "bar") ; tfn ! ("foo" , "" , "") ; tfn ! ("" , "foo" , "foo") ; tfn ! ("." , "foo" , "./foo") ; tfn ! ("foo/" , "bar" , "bar") ; tfn ! ("foo/." , "bar" , "bar") ; tfn ! (".." , "foo" , "../foo") ; tfn ! ("foo/.." , "bar" , "foo/../bar") ; tfn ! ("/" , "foo" , "/foo") ; }
};
}
