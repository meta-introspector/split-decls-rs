// Generated macro for queue_single_threaded (function)
macro_rules! Depcrate_examplesqueue_single_threaded {
() => {
// Module: crate::examples
// Provides: {"queue_single_threaded"}
// Dependencies: {}
# [test] fn queue_single_threaded () { let workload_size = 256 ; let queue : Queue < isize > = Queue :: default () ; for i in 1 .. workload_size { queue . push (i) ; } let mut expected = 1 ; while let Some (popped) = queue . pop () { assert_eq ! (** popped , expected) ; expected = * * popped + 1 ; } assert_eq ! (expected , workload_size) ; assert ! (queue . is_empty ()) ; }
};
}
