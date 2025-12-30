// Generated macro for test_try_from_str_inference (function)
macro_rules! Depcrate_frontendtest_try_from_str_inference {
() => {
// Module: crate::frontend
// Provides: {"test_try_from_str_inference"}
// Dependencies: {}
# [test] fn test_try_from_str_inference () { use crate :: SinglePlaceholder ; let _ : Box < Pattern < SinglePlaceholder > > = Pattern :: try_from_str ("{0} days" , Default :: default ()) . unwrap () ; let _ = Pattern :: < SinglePlaceholder > :: try_from_str ("{0} days" , Default :: default ()) . unwrap () ; }
};
}
