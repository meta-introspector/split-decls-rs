// Generated macro for test_map (function)
macro_rules! Depcrate_ser_teststest_map {
() => {
// Module: crate::ser::tests
// Provides: {"test_map"}
// Dependencies: {}
# [test] fn test_map () { use alloc :: collections :: BTreeMap ; let mut map = BTreeMap :: new () ; map . insert ((true , false) , 4) ; map . insert ((false , false) , 123) ; check_to_string_writer (& map , "{(false,false):123,(true,false):4}" , "{\n    (false, false): 123,\n    (true, false): 4,\n}" ,) ; }
};
}
