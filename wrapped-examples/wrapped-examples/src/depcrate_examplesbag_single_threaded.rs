// Generated macro for bag_single_threaded (function)
macro_rules! Depcrate_examplesbag_single_threaded {
() => {
// Module: crate::examples
// Provides: {"bag_single_threaded"}
// Dependencies: {}
# [test] fn bag_single_threaded () { let workload_size = 256 ; let bag : Bag < isize > = Bag :: default () ; for i in 1 .. workload_size { bag . push (i) ; } let mut num_popped = 0 ; while let Some (popped) = bag . pop () { assert_ne ! (popped , 0) ; assert ! (popped < workload_size) ; num_popped += 1 ; } assert_eq ! (num_popped , workload_size - 1) ; assert ! (bag . is_empty ()) ; }
};
}
