// Generated macro for sample_dict_mut (function)
macro_rules! Depcrate_tests_mutable_dictionarysample_dict_mut {
() => {
// Module: crate::tests::mutable_dictionary
// Provides: {"sample_dict_mut"}
// Dependencies: {}
fn sample_dict_mut () -> Retained < NSMutableDictionary < NSNumber , NSMutableDictionary > > { NSMutableDictionary :: from_retained_objects (& [& * NSNumber :: new_i32 (1) , & * NSNumber :: new_i32 (2) , & * NSNumber :: new_i32 (3) ,] , & [NSMutableDictionary :: new () , NSMutableDictionary :: new () , NSMutableDictionary :: new () ,] ,) }
};
}
