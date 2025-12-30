// Generated macro for overflow_u8 (function)
macro_rules! Depcrate_integer_testsoverflow_u8 {
() => {
// Module: crate::integer::tests
// Provides: {"overflow_u8"}
// Dependencies: {}
# [test] fn overflow_u8 () { let inputs = ["256" , "0x100" , "0o400" , "0b100000000" , "257" , "0x101" , "0o401" , "0b100000001" , "300" , "1548" , "2548985" , "256u128" , "256u8" , "2_5_6" , "256_____1" , "256__" ,] ; for & input in & inputs { let lit = IntegerLit :: parse (input) . expect ("failed to parse") ; assert ! (lit . value ::< u8 > () . is_none ()) ; } }
};
}
