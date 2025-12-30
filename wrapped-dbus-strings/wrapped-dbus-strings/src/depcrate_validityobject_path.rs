// Generated macro for object_path (function)
macro_rules! Depcrate_validityobject_path {
() => {
// Module: crate::validity
// Provides: {"object_path"}
// Dependencies: {}
# [test] fn object_path () { assert ! (is_valid_object_path (b"") . is_err ()) ; assert ! (is_valid_object_path (b"/") . is_ok ()) ; assert ! (is_valid_object_path (b"/1234") . is_ok ()) ; assert ! (is_valid_object_path (b"/abce/") . is_err ()) ; assert ! (is_valid_object_path (b"/ab//c/d") . is_err ()) ; assert ! (is_valid_object_path (b"/a/c/df1") . is_ok ()) ; assert ! (is_valid_object_path (b"/12.43/fasd") . is_err ()) ; assert ! (is_valid_object_path (b"/asdf/_123") . is_ok ()) ; }
};
}
