// Generated macro for tests (module)
macro_rules! Depcrate_streamtests {
() => {
// Module: crate::stream
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [inline] fn uncons_range_at_end () { assert_eq ! ("" . uncons_range (0) , Ok ("")) ; assert_eq ! ("123" . uncons_range (3) , Ok ("123")) ; assert_eq ! ((& [1] [..]) . uncons_range (1) , Ok (& [1] [..])) ; let s : & [u8] = & [] ; assert_eq ! (SliceStream (s) . uncons_range (0) , Ok (& [] [..])) ; } # [test] fn larger_than_1_byte_items_return_correct_distance () { let mut input = & [123i32 , 0i32] [..] ; let before = input . checkpoint () ; assert_eq ! (input . distance (& before) , 0) ; input . uncons () . unwrap () ; assert_eq ! (input . distance (& before) , 1) ; input . uncons () . unwrap () ; assert_eq ! (input . distance (& before) , 2) ; input . reset (before) . unwrap () ; assert_eq ! (input . distance (& before) , 0) ; } }
};
}
