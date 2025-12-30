// Generated macro for test_overlapping (function)
macro_rules! Depcrate_matches_overlapping_armstest_overlapping {
() => {
// Module: crate::matches::overlapping_arms
// Provides: {"test_overlapping"}
// Dependencies: {}
# [test] fn test_overlapping () { use rustc_span :: DUMMY_SP ; let sp = | s , e | SpannedRange { span : DUMMY_SP , node : (s , e) , } ; assert_eq ! (None , overlapping ::< u8 > (& [])) ; assert_eq ! (None , overlapping (& [sp (1 , EndBound :: Included (4))])) ; assert_eq ! (None , overlapping (& [sp (1 , EndBound :: Included (4)) , sp (5 , EndBound :: Included (6))])) ; assert_eq ! (None , overlapping (& [sp (1 , EndBound :: Included (4)) , sp (5 , EndBound :: Included (6)) , sp (10 , EndBound :: Included (11))] ,)) ; assert_eq ! (Some ((& sp (1 , EndBound :: Included (4)) , & sp (3 , EndBound :: Included (6)))) , overlapping (& [sp (1 , EndBound :: Included (4)) , sp (3 , EndBound :: Included (6))])) ; assert_eq ! (Some ((& sp (5 , EndBound :: Included (6)) , & sp (6 , EndBound :: Included (11)))) , overlapping (& [sp (1 , EndBound :: Included (4)) , sp (5 , EndBound :: Included (6)) , sp (6 , EndBound :: Included (11))] ,)) ; }
};
}
