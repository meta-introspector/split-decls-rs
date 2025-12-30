// Generated macro for test (module)
macro_rules! Depcrate_coord_ranged1d_types_slicetest {
() => {
// Module: crate::coord::ranged1d::types::slice
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_slice_range () { let my_slice = [1 , 2 , 3 , 0 , - 1 , - 2] ; let slice_range : RangedSlice < i32 > = my_slice [..] . into () ; assert_eq ! (slice_range . range () , & 1 ..&- 2) ; assert_eq ! (slice_range . key_points (6) , my_slice . iter () . collect ::< Vec < _ >> ()) ; assert_eq ! (slice_range . map (&& 0 , (0 , 50)) , 30) ; } # [test] fn test_slice_range_discrete () { let my_slice = [1 , 2 , 3 , 0 , - 1 , - 2] ; let slice_range : RangedSlice < i32 > = my_slice [..] . into () ; assert_eq ! (slice_range . size () , 6) ; assert_eq ! (slice_range . index_of (&& 3) , Some (2)) ; assert_eq ! (slice_range . from_index (2) , Some (& 3)) ; } }
};
}
