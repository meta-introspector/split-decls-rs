// Generated macro for test_enum (function)
macro_rules! Depcrate_de_teststest_enum {
() => {
// Module: crate::de::tests
// Provides: {"test_enum"}
// Dependencies: {}
# [test] fn test_enum () { check_from_str_bytes_reader ("A" , Ok (MyEnum :: A)) ; check_from_str_bytes_reader ("B(true,)" , Ok (MyEnum :: B (true))) ; check_from_str_bytes_reader :: < MyEnum > ("B" , Err (SpannedError { code : Error :: ExpectedStructLike , span : Span { start : Position { line : 1 , col : 1 } , end : Position { line : 1 , col : 2 } , } , }) ,) ; check_from_str_bytes_reader ("C(true,3.5,)" , Ok (MyEnum :: C (true , 3.5))) ; check_from_str_bytes_reader ("D(a:2,b:3,)" , Ok (MyEnum :: D { a : 2 , b : 3 })) ; }
};
}
