// Generated macro for tuple_test (function)
macro_rules! Depcrate_sequence_teststuple_test {
() => {
// Module: crate::sequence::tests
// Provides: {"tuple_test"}
// Dependencies: {}
# [test] fn tuple_test () { # [allow (clippy :: type_complexity)] fn tuple_3 (i : & [u8]) -> IResult < & [u8] , (u16 , & [u8] , & [u8]) > { crate :: Parser :: parse (& mut (be_u16 , take (3u8) , tag ("fg")) , i) } assert_eq ! (tuple_3 (& b"abcdefgh" [..]) , Ok ((& b"h" [..] , (0x6162u16 , & b"cde" [..] , & b"fg" [..])))) ; assert_eq ! (tuple_3 (& b"abcd" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (tuple_3 (& b"abcde" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (tuple_3 (& b"abcdejk" [..]) , Err (Err :: Error (error_position ! (& b"jk" [..] , ErrorKind :: Tag)))) ; }
};
}
