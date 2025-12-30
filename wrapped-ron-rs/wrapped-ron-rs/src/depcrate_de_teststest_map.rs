// Generated macro for test_map (function)
macro_rules! Depcrate_de_teststest_map {
() => {
// Module: crate::de::tests
// Provides: {"test_map"}
// Dependencies: {}
# [cfg (feature = "std")] # [test] fn test_map () { use std :: collections :: HashMap ; let mut map = HashMap :: new () ; map . insert ((true , false) , 4) ; map . insert ((false , false) , 123) ; check_from_str_bytes_reader ("{
        (true,false,):4,
        (false,false,):123,
    }" , Ok (map) ,) ; }
};
}
