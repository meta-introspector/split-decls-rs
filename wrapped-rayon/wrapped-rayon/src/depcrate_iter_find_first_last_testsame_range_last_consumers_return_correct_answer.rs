// Generated macro for same_range_last_consumers_return_correct_answer (function)
macro_rules! Depcrate_iter_find_first_last_testsame_range_last_consumers_return_correct_answer {
() => {
// Module: crate::iter::find_first_last::test
// Provides: {"same_range_last_consumers_return_correct_answer"}
// Dependencies: {}
# [test] fn same_range_last_consumers_return_correct_answer () { let find_op = | x : & i32 | x % 2 == 0 ; let last_found = AtomicUsize :: new (0) ; let consumer = FindConsumer :: new (& find_op , MatchPosition :: Rightmost , & last_found) ; let far_left_consumer = consumer . split_off_left () ; for _ in 0 .. usize :: BITS { consumer . split_off_left () ; } let reducer = consumer . to_reducer () ; let consumer = consumer . split_off_left () ; let left_folder = consumer . split_off_left () . into_folder () ; let right_folder = consumer . into_folder () ; let right_folder = right_folder . consume (2) . consume (3) ; assert_eq ! (left_folder . boundary , right_folder . boundary) ; assert ! (! left_folder . full ()) ; assert ! (far_left_consumer . full ()) ; let left_folder = left_folder . consume (0) . consume (1) ; assert_eq ! (reducer . reduce (left_folder . complete () , right_folder . complete ()) , Some (2)) ; }
};
}
