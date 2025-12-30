// Generated macro for one_of_test (function)
macro_rules! Depcrate_character_testsone_of_test {
() => {
// Module: crate::character::tests
// Provides: {"one_of_test"}
// Dependencies: {}
# [test] fn one_of_test () { fn f (i : & [u8]) -> IResult < & [u8] , char > { one_of ("ab") (i) } let a = & b"abcd" [..] ; assert_eq ! (f (a) , Ok ((& b"bcd" [..] , 'a'))) ; let b = & b"cde" [..] ; assert_eq ! (f (b) , Err (Err :: Error (error_position ! (b , ErrorKind :: OneOf)))) ; fn utf8 (i : & str) -> IResult < & str , char > { one_of ("+\u{FF0B}") (i) } assert ! (utf8 ("+") . is_ok ()) ; assert ! (utf8 ("\u{FF0B}") . is_ok ()) ; }
};
}
