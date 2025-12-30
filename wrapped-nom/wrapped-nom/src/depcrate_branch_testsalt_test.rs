// Generated macro for alt_test (function)
macro_rules! Depcrate_branch_testsalt_test {
() => {
// Module: crate::branch::tests
// Provides: {"alt_test"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [test] fn alt_test () { use crate :: Parser ; fn work (input : & [u8]) -> IResult < & [u8] , & [u8] , ErrorStr > { Ok ((& b"" [..] , input)) } # [allow (unused_variables)] fn dont_work (input : & [u8]) -> IResult < & [u8] , & [u8] , ErrorStr > { Err (Err :: Error (ErrorStr ("abcd" . to_string ()))) } fn work2 (input : & [u8]) -> IResult < & [u8] , & [u8] , ErrorStr > { Ok ((input , & b"" [..])) } fn alt1 (i : & [u8]) -> IResult < & [u8] , & [u8] , ErrorStr > { alt ((dont_work , dont_work)) . parse (i) } fn alt2 (i : & [u8]) -> IResult < & [u8] , & [u8] , ErrorStr > { alt ((dont_work , work)) . parse (i) } fn alt3 (i : & [u8]) -> IResult < & [u8] , & [u8] , ErrorStr > { alt ((dont_work , dont_work , work2 , dont_work)) . parse (i) } let a = & b"abcd" [..] ; assert_eq ! (alt1 (a) , Err (Err :: Error (error_node_position ! (a , ErrorKind :: Alt , ErrorStr ("abcd" . to_string ()))))) ; assert_eq ! (alt2 (a) , Ok ((& b"" [..] , a))) ; assert_eq ! (alt3 (a) , Ok ((a , & b"" [..]))) ; fn alt4 (i : & [u8]) -> IResult < & [u8] , & [u8] > { alt ((tag ("abcd") , tag ("efgh"))) . parse (i) } let b = & b"efgh" [..] ; assert_eq ! (alt4 (a) , Ok ((& b"" [..] , a))) ; assert_eq ! (alt4 (b) , Ok ((& b"" [..] , b))) ; }
};
}
