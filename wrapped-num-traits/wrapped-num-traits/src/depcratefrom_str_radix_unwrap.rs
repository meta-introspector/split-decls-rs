// Generated macro for from_str_radix_unwrap (function)
macro_rules! Depcratefrom_str_radix_unwrap {
() => {
// Module: crate
// Provides: {"from_str_radix_unwrap"}
// Dependencies: {}
# [test] fn from_str_radix_unwrap () { let i : i32 = Num :: from_str_radix ("0" , 10) . unwrap () ; assert_eq ! (i , 0) ; let f : f32 = Num :: from_str_radix ("0.0" , 10) . unwrap () ; assert_eq ! (f , 0.0) ; }
};
}
