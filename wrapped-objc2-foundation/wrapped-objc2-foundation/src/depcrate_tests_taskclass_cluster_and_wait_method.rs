// Generated macro for class_cluster_and_wait_method (function)
macro_rules! Depcrate_tests_taskclass_cluster_and_wait_method {
() => {
// Module: crate::tests::task
// Provides: {"class_cluster_and_wait_method"}
// Dependencies: {}
# [test] # [cfg_attr (not (target_vendor = "apple") , ignore = "only on Apple")] fn class_cluster_and_wait_method () { let sel = sel ! (waitUntilExit) ; let method = NSTask :: class () . instance_method (sel) ; assert ! (method . is_none () , "class does not have method") ; let task = NSTask :: new () ; assert ! (task . respondsToSelector (sel) , "object has method") ; }
};
}
