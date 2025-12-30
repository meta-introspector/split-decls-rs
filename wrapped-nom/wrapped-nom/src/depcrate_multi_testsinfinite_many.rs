// Generated macro for infinite_many (function)
macro_rules! Depcrate_multi_testsinfinite_many {
() => {
// Module: crate::multi::tests
// Provides: {"infinite_many"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] fn infinite_many () { fn tst (input : & [u8]) -> IResult < & [u8] , & [u8] > { Err (Err :: Error (error_position ! (input , ErrorKind :: Tag))) } fn multi0 (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { many0 (tst) . parse (i) } let a = & b"abcdef" [..] ; assert_eq ! (multi0 (a) , Ok ((a , Vec :: new ()))) ; fn multi1 (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { many1 (tst) . parse (i) } let a = & b"abcdef" [..] ; assert_eq ! (multi1 (a) , Err (Err :: Error (error_position ! (a , ErrorKind :: Tag)))) ; }
};
}
