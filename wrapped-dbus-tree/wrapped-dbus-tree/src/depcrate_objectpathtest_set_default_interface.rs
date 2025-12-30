// Generated macro for test_set_default_interface (function)
macro_rules! Depcrate_objectpathtest_set_default_interface {
() => {
// Module: crate::objectpath
// Provides: {"test_set_default_interface"}
// Dependencies: {}
# [test] fn test_set_default_interface () { let iface_name : IfaceName < '_ > = "com.example.echo" . into () ; let f = super :: Factory :: new_fn :: < () > () ; let t = f . object_path ("/echo" , ()) . default_interface (iface_name . clone ()) ; assert_eq ! (t . default_iface , Some (iface_name)) ; }
};
}
