// Generated macro for char_byteslice (function)
macro_rules! Depcrate_character_testschar_byteslice {
() => {
// Module: crate::character::tests
// Provides: {"char_byteslice"}
// Dependencies: {}
# [test] fn char_byteslice () { fn f (i : & [u8]) -> IResult < & [u8] , char > { char ('c') (i) } let a = & b"abcd" [..] ; assert_eq ! (f (a) , Err (Err :: Error (error_position ! (a , ErrorKind :: Char)))) ; let b = & b"cde" [..] ; assert_eq ! (f (b) , Ok ((& b"de" [..] , 'c'))) ; }
};
}
