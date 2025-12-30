// Generated macro for tests (module)
macro_rules! Depcrate_traitstests {
() => {
// Module: crate::traits
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_offset_u8 () { let s = b"abcd123" ; let a = & s [..] ; let b = & a [2 ..] ; let c = & a [.. 4] ; let d = & a [3 .. 5] ; assert_eq ! (a . offset (b) , 2) ; assert_eq ! (a . offset (c) , 0) ; assert_eq ! (a . offset (d) , 3) ; } # [test] fn test_offset_str () { let a = "abcřèÂßÇd123" ; let b = & a [7 ..] ; let c = & a [.. 5] ; let d = & a [5 .. 9] ; assert_eq ! (a . offset (b) , 7) ; assert_eq ! (a . offset (c) , 0) ; assert_eq ! (a . offset (d) , 5) ; } # [test] fn test_slice_index () { let a = "abcřèÂßÇd123" ; assert_eq ! (a . slice_index (0) , Ok (0)) ; assert_eq ! (a . slice_index (2) , Ok (2)) ; } # [test] fn test_slice_index_utf8 () { let a = "a¡€💢€¡a" ; for (c , len) in a . chars () . zip ([1 , 2 , 3 , 4 , 3 , 2 , 1]) { assert_eq ! (c . len () , len) ; } assert_eq ! (a . slice_index (0) , Ok (0)) ; assert_eq ! (a . slice_index (1) , Ok (1)) ; assert_eq ! (a . slice_index (2) , Ok (3)) ; assert_eq ! (a . slice_index (3) , Ok (6)) ; assert_eq ! (a . slice_index (4) , Ok (10)) ; assert_eq ! (a . slice_index (5) , Ok (13)) ; assert_eq ! (a . slice_index (6) , Ok (15)) ; assert_eq ! (a . slice_index (7) , Ok (16)) ; assert ! (a . slice_index (8) . is_err ()) ; } }
};
}
