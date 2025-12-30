// Generated macro for test (module)
macro_rules! Depcrate_parser_statetest {
() => {
// Module: crate::parser_state
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn normalize_index_pos () { assert_eq ! (normalize_index (4 , 6) , Some (4)) ; assert_eq ! (normalize_index (5 , 5) , Some (5)) ; assert_eq ! (normalize_index (6 , 3) , None) ; } # [test] fn normalize_index_neg () { assert_eq ! (normalize_index (- 4 , 6) , Some (2)) ; assert_eq ! (normalize_index (- 5 , 5) , Some (0)) ; assert_eq ! (normalize_index (- 6 , 3) , None) ; } }
};
}
