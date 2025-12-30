// Generated macro for test (module)
macro_rules! Depcrate_rngs_threadtest {
() => {
// Module: crate::rngs::thread
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] fn test_thread_rng () { use crate :: Rng ; let mut r = crate :: rng () ; r . random :: < i32 > () ; assert_eq ! (r . random_range (0 .. 1) , 0) ; } # [test] fn test_debug_output () { assert_eq ! (std :: format ! ("{:?}" , crate :: rng ()) , "ThreadRng { .. }") ; } }
};
}
