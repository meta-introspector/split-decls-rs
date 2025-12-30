// Generated macro for dict_from_mutable (function)
macro_rules! Depcrate_tests_mutable_dictionarydict_from_mutable {
() => {
// Module: crate::tests::mutable_dictionary
// Provides: {"dict_from_mutable"}
// Dependencies: {}
# [test] # [cfg (feature = "NSString")] fn dict_from_mutable () { use crate :: { NSMutableString , NSString } ; let _ : Retained < NSMutableDictionary < NSString , NSString > > = NSMutableDictionary :: from_slices (& [& * NSMutableString :: from_str ("a")] , & [& * * NSMutableString :: from_str ("b")] ,) ; }
};
}
