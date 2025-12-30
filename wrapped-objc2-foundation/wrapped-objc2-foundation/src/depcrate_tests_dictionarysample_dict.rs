// Generated macro for sample_dict (function)
macro_rules! Depcrate_tests_dictionarysample_dict {
() => {
// Module: crate::tests::dictionary
// Provides: {"sample_dict"}
// Dependencies: {}
fn sample_dict (key : & str) -> Retained < NSDictionary < NSString , NSObject > > { let string = NSString :: from_str (key) ; let obj = NSObject :: new () ; NSDictionary :: from_retained_objects (& [& * string] , & [obj]) }
};
}
