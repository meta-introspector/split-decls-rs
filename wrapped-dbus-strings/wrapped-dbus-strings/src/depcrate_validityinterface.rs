// Generated macro for interface (function)
macro_rules! Depcrate_validityinterface {
() => {
// Module: crate::validity
// Provides: {"interface"}
// Dependencies: {}
# [test] fn interface () { assert ! (is_valid_interface_name (b"") . is_err ()) ; assert ! (is_valid_interface_name (b"He11o") . is_err ()) ; assert ! (is_valid_interface_name (b"Hello.") . is_err ()) ; assert ! (is_valid_interface_name (b"Hello!.World") . is_err ()) ; assert ! (is_valid_interface_name (b"ZZZ.1Hello") . is_err ()) ; assert ! (is_valid_interface_name (b"Hello.W0rld") . is_ok ()) ; assert ! (is_valid_interface_name (b":1.54") . is_err ()) ; }
};
}
