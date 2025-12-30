// Generated macro for class_cluster_and_init_method (function)
macro_rules! Depcrate_tests_stringclass_cluster_and_init_method {
() => {
// Module: crate::tests::string
// Provides: {"class_cluster_and_init_method"}
// Dependencies: {}
# [test] # [cfg_attr (not (target_vendor = "apple") , ignore = "only on Apple")] fn class_cluster_and_init_method () { let sel = sel ! (initWithBytes : length : encoding :) ; let method = NSString :: class () . instance_method (sel) ; assert ! (method . is_none () , "class does not have method") ; let s = NSString :: from_str ("foo") ; assert ! (! s . respondsToSelector (sel) , "object does not have method") ; let allocated_object : * mut NSString = unsafe { msg_send ! [NSString :: class () , alloc] } ; let has_method : bool = unsafe { msg_send ! [allocated_object , respondsToSelector : sel] } ; let _ : () = unsafe { msg_send ! [allocated_object , release] } ; assert ! (has_method , "Allocated (but uninitialized) has method") ; }
};
}
