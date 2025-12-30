// Generated macro for test_enum (function)
macro_rules! Depcrate_ser_teststest_enum {
() => {
// Module: crate::ser::tests
// Provides: {"test_enum"}
// Dependencies: {}
# [test] fn test_enum () { check_to_string_writer (& MyEnum :: A , "A" , "A") ; check_to_string_writer (& MyEnum :: B (true) , "B(true)" , "B(true)") ; check_to_string_writer (& MyEnum :: C (true , 3.5) , "C(true,3.5)" , "C(true, 3.5)") ; check_to_string_writer (& MyEnum :: D { a : 2 , b : 3 } , "D(a:2,b:3)" , "D(a: 2, b: 3)") ; }
};
}
