// Generated macro for test_push (function)
macro_rules! Depcrate_teststest_push {
() => {
// Module: crate::tests
// Provides: {"test_push"}
// Dependencies: {}
# [test] pub fn test_push () { macro_rules ! tp (($ path : expr , $ push : expr , $ expected : expr) => ({ let mut actual = RelativePathBuf :: from ($ path) ; actual . push ($ push) ; assert ! (actual . as_str () == $ expected , "pushing {:?} onto {:?}: Expected {:?}, got {:?}" , $ push , $ path , $ expected , actual . as_str ()) ; }) ;) ; tp ! ("" , "foo" , "foo") ; tp ! ("foo" , "bar" , "foo/bar") ; tp ! ("foo/" , "bar" , "foo/bar") ; tp ! ("foo//" , "bar" , "foo//bar") ; tp ! ("foo/." , "bar" , "foo/./bar") ; tp ! ("foo./." , "bar" , "foo././bar") ; tp ! ("foo" , "" , "foo/") ; tp ! ("foo" , "." , "foo/.") ; tp ! ("foo" , ".." , "foo/..") ; }
};
}
