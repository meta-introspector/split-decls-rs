// Generated macro for test (module)
macro_rules! Depcrate_coord_ranged1d_discretetest {
() => {
// Module: crate::coord::ranged1d::discrete
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_value_iter () { let range : crate :: coord :: ranged1d :: types :: RangedCoordi32 = (- 10 .. 10) . into () ; let values : Vec < _ > = range . values () . collect () ; assert_eq ! (21 , values . len ()) ; for (expected , value) in (- 10 ..= 10) . zip (values) { assert_eq ! (expected , value) ; } assert_eq ! (range . next (& 5) , Some (6)) ; assert_eq ! (range . next (& 10) , None) ; assert_eq ! (range . previous (&- 10) , None) ; assert_eq ! (range . previous (& 10) , Some (9)) ; } # [test] fn test_centric_coord () { let coord = (0 .. 10) . into_segmented () ; assert_eq ! (coord . size () , 12) ; for i in 0 ..= 11 { match coord . from_index (i as usize) { Some (SegmentValue :: Exact (value)) => assert_eq ! (i , value) , Some (SegmentValue :: Last) => assert_eq ! (i , 11) , _ => panic ! () , } } for (kps , idx) in coord . key_points (20) . into_iter () . zip (0 ..) { match kps { SegmentValue :: CenterOf (value) if value <= 10 => assert_eq ! (value , idx) , _ => panic ! () , } } assert_eq ! (coord . map (& SegmentValue :: CenterOf (0) , (0 , 24)) , 1) ; assert_eq ! (coord . map (& SegmentValue :: Exact (0) , (0 , 24)) , 0) ; assert_eq ! (coord . map (& SegmentValue :: Exact (1) , (0 , 24)) , 2) ; } }
};
}
