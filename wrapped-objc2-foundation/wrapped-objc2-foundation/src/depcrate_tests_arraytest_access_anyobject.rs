// Generated macro for test_access_anyobject (function)
macro_rules! Depcrate_tests_arraytest_access_anyobject {
() => {
// Module: crate::tests::array
// Provides: {"test_access_anyobject"}
// Dependencies: {}
# [test] fn test_access_anyobject () { let obj : Retained < AnyObject > = NSObject :: new () . into_super () ; let array = NSArray :: from_retained_slice (& [obj . clone () , obj . clone ()]) ; assert ! (ptr :: eq (&* array . objectAtIndex (0) , &* obj)) ; assert ! (ptr :: eq (unsafe { array . objectAtIndex_unchecked (0) } , &* obj)) ; for _ in array . iter () { } for _ in unsafe { array . iter_unchecked () } { } for _ in array { } }
};
}
