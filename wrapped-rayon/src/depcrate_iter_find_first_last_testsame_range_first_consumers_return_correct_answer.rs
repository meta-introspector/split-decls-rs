// Generated macro for same_range_first_consumers_return_correct_answer (function)
macro_rules! Depcrate_iter_find_first_last_testsame_range_first_consumers_return_correct_answer {
() => {
// Module: crate::iter::find_first_last::test
// Provides: {"same_range_first_consumers_return_correct_answer"}
// Dependencies: {}
# [test] fn same_range_first_consumers_return_correct_answer () { let find_op = | x : & i32 | x % 2 == 0 ; let first_found = AtomicUsize :: new (usize :: MAX) ; let far_right_consumer = FindConsumer :: new (& find_op , MatchPosition :: Leftmost , & first_found) ; let consumer = far_right_consumer . split_off_left () ; for _ in 0 .. usize :: BITS { consumer . split_off_left () ; } let reducer = consumer . to_reducer () ; let left_folder = consumer . split_off_left () . into_folder () ; let right_folder = consumer . into_folder () ; let left_folder = left_folder . consume (0) . consume (1) ; assert_eq ! (left_folder . boundary , right_folder . boundary) ; assert ! (! right_folder . full ()) ; assert ! (far_right_consumer . full ()) ; let right_folder = right_folder . consume (2) . consume (3) ; assert_eq ! (reducer . reduce (left_folder . complete () , right_folder . complete ()) , Some (0)) ; }
};
}
