// Generated macro for round_up_to_alignment_works (function)
macro_rules! Depcrate_transforms_multi_value_testsround_up_to_alignment_works {
() => {
// Module: crate::transforms::multi_value::tests
// Provides: {"round_up_to_alignment_works"}
// Dependencies: {}
# [test] fn round_up_to_alignment_works () { for & (n , align , expected) in & [(0 , 1 , 0) , (1 , 1 , 1) , (2 , 1 , 2) , (0 , 2 , 0) , (1 , 2 , 2) , (2 , 2 , 2) , (3 , 2 , 4) , (0 , 4 , 0) , (1 , 4 , 4) , (2 , 4 , 4) , (3 , 4 , 4) , (4 , 4 , 4) , (5 , 4 , 8) ,] { let actual = super :: round_up_to_alignment (n , align) ; println ! ("round_up_to_alignment(n = {n}, align = {align}) = {actual} (expected {expected})") ; assert_eq ! (actual , expected) ; } }
};
}
