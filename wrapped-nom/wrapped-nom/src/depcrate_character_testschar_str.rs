// Generated macro for char_str (function)
macro_rules! Depcrate_character_testschar_str {
() => {
// Module: crate::character::tests
// Provides: {"char_str"}
// Dependencies: {}
# [test] fn char_str () { fn f (i : & str) -> IResult < & str , char > { char ('c') (i) } let a = "abcd" ; assert_eq ! (f (a) , Err (Err :: Error (error_position ! (a , ErrorKind :: Char)))) ; let b = "cde" ; assert_eq ! (f (b) , Ok (("de" , 'c'))) ; }
};
}
