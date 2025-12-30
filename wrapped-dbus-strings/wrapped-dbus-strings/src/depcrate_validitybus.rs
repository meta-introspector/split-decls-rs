// Generated macro for bus (function)
macro_rules! Depcrate_validitybus {
() => {
// Module: crate::validity
// Provides: {"bus"}
// Dependencies: {}
# [test] fn bus () { assert ! (is_valid_bus_name (b"") . is_err ()) ; assert ! (is_valid_bus_name (b"He11o") . is_err ()) ; assert ! (is_valid_bus_name (b"Hello.") . is_err ()) ; assert ! (is_valid_bus_name (b"Hello!.World") . is_err ()) ; assert ! (is_valid_bus_name (b"ZZZ.1Hello") . is_err ()) ; assert ! (is_valid_bus_name (b"Hello.W0rld") . is_ok ()) ; assert ! (is_valid_bus_name (b":1.54") . is_ok ()) ; assert ! (is_valid_bus_name (b"1.54") . is_err ()) ; }
};
}
