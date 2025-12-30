// Generated macro for member (function)
macro_rules! Depcrate_validitymember {
() => {
// Module: crate::validity
// Provides: {"member"}
// Dependencies: {}
# [test] fn member () { assert ! (is_valid_member_name (b"") . is_err ()) ; assert ! (is_valid_member_name (b"He11o") . is_ok ()) ; assert ! (is_valid_member_name (b"He11o!") . is_err ()) ; assert ! (is_valid_member_name (b"1Hello") . is_err ()) ; assert ! (is_valid_member_name (b":1.54") . is_err ()) ; }
};
}
