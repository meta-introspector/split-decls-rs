// Generated macro for test_comment (function)
macro_rules! Depcrate_de_teststest_comment {
() => {
// Module: crate::de::tests
// Provides: {"test_comment"}
// Dependencies: {}
# [test] fn test_comment () { check_from_str_bytes_reader ("(
x: 1.0, // x is just 1
// There is another comment in the very next line..
// And y is indeed
y: 2.0 // 2!
    )" , Ok (MyStruct { x : 1.0 , y : 2.0 }) ,) ; }
};
}
