// Generated macro for many0_count_test (function)
macro_rules! Depcrate_multi_testsmany0_count_test {
() => {
// Module: crate::multi::tests
// Provides: {"many0_count_test"}
// Dependencies: {}
# [test] fn many0_count_test () { fn count0_nums (i : & [u8]) -> IResult < & [u8] , usize > { many0_count (pair (digit , tag (","))) . parse (i) } assert_eq ! (count0_nums (& b"123,junk" [..]) , Ok ((& b"junk" [..] , 1))) ; assert_eq ! (count0_nums (& b"123,45,junk" [..]) , Ok ((& b"junk" [..] , 2))) ; assert_eq ! (count0_nums (& b"1,2,3,4,5,6,7,8,9,0,junk" [..]) , Ok ((& b"junk" [..] , 10))) ; assert_eq ! (count0_nums (& b"hello" [..]) , Ok ((& b"hello" [..] , 0))) ; }
};
}
