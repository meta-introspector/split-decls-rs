// Generated macro for test_from_base_class (function)
macro_rules! Depcrate_tests_dictionarytest_from_base_class {
() => {
// Module: crate::tests::dictionary
// Provides: {"test_from_base_class"}
// Dependencies: {}
fn test_from_base_class (cls : & AnyClass) { type Base = NSString ; let obj1 : Retained < Base > = unsafe { Retained :: from_raw (ffi :: class_createInstance (cls , 0) . cast ()) . unwrap () } ; let obj2 : Retained < Base > = unsafe { Retained :: from_raw (ffi :: class_createInstance (cls , 0) . cast ()) . unwrap () } ; let _dict = NSDictionary :: from_retained_objects (& [& * obj1 , & * obj2] , & [NSObject :: new () , NSObject :: new ()]) ; }
};
}
