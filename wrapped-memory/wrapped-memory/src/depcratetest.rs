// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_byte_extraction () { let log = ["dhat: Total:     20,122 bytes in 129 blocks" . to_owned () , "dhat: At t-gmax: 9,328 bytes in 90 blocks" . to_owned () , "dhat: At t-end:  0 bytes in 0 blocks" . to_owned () , "dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html" . to_owned () ,] ; let (total , gmax , end) = parse_dhat_log (& log) ; assert_eq ! (total , 20122) ; assert_eq ! (gmax , 9328) ; assert_eq ! (end , 0) ; } }
};
}
