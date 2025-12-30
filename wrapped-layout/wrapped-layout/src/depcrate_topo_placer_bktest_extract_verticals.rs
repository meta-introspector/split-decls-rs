// Generated macro for test_extract_verticals (function)
macro_rules! Depcrate_topo_placer_bktest_extract_verticals {
() => {
// Module: crate::topo::placer::bk
// Provides: {"test_extract_verticals"}
// Dependencies: {}
# [test] fn test_extract_verticals () { let mut ai = NodeAttachInfo :: new (6) ; ai . add (NodeHandle :: new (0) , NodeHandle :: new (1)) ; ai . add (NodeHandle :: new (1) , NodeHandle :: new (2)) ; ai . add (NodeHandle :: new (2) , NodeHandle :: new (3)) ; ai . add (NodeHandle :: new (4) , NodeHandle :: new (5)) ; let verticals = ai . get_verticals () ; assert_eq ! (verticals . len () , 2) ; assert_eq ! (verticals [0] [0] . get_index () , 0) ; assert_eq ! (verticals [0] [1] . get_index () , 1) ; assert_eq ! (verticals [0] [2] . get_index () , 2) ; assert_eq ! (verticals [0] [3] . get_index () , 3) ; assert_eq ! (verticals [1] [0] . get_index () , 4) ; assert_eq ! (verticals [1] [1] . get_index () , 5) ; }
};
}
