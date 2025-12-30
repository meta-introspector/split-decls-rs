// Generated macro for test (module)
macro_rules! Depcrate_data_quartilestest {
() => {
// Module: crate::data::quartiles
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] # [should_panic] fn test_empty_input () { let empty_array : [i32 ; 0] = [] ; Quartiles :: new (& empty_array) ; } # [test] fn test_low_inputs () { assert_eq ! (Quartiles :: new (& [15.0]) . values () , [15.0 , 15.0 , 15.0 , 15.0 , 15.0]) ; assert_eq ! (Quartiles :: new (& [10 , 20]) . values () , [5.0 , 12.5 , 15.0 , 17.5 , 25.0]) ; assert_eq ! (Quartiles :: new (& [10 , 20 , 30]) . values () , [0.0 , 15.0 , 20.0 , 25.0 , 40.0]) ; } }
};
}
