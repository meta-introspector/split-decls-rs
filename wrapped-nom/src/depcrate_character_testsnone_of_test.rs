// Generated macro for none_of_test (function)
macro_rules! Depcrate_character_testsnone_of_test {
() => {
// Module: crate::character::tests
// Provides: {"none_of_test"}
// Dependencies: {}
# [test] fn none_of_test () { fn f (i : & [u8]) -> IResult < & [u8] , char > { none_of ("ab") (i) } let a = & b"abcd" [..] ; assert_eq ! (f (a) , Err (Err :: Error (error_position ! (a , ErrorKind :: NoneOf)))) ; let b = & b"cde" [..] ; assert_eq ! (f (b) , Ok ((& b"de" [..] , 'c'))) ; }
};
}
