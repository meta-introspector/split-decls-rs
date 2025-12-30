// Generated macro for tests (module)
macro_rules! Depcrate_recovery_congestiontests {
() => {
// Module: crate::recovery::congestion
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn ssthresh_init () { let ssthresh : SsThresh = Default :: default () ; assert_eq ! (ssthresh . get () , usize :: MAX) ; assert_eq ! (ssthresh . startup_exit () , None) ; } # [test] fn ssthresh_in_css () { let expected_startup_exit = StartupExit :: new (1000 , None , StartupExitReason :: PersistentQueue) ; let mut ssthresh : SsThresh = Default :: default () ; ssthresh . update (1000 , true) ; assert_eq ! (ssthresh . get () , 1000) ; assert_eq ! (ssthresh . startup_exit () , Some (expected_startup_exit)) ; ssthresh . update (2000 , true) ; assert_eq ! (ssthresh . get () , 2000) ; assert_eq ! (ssthresh . startup_exit () , Some (expected_startup_exit)) ; ssthresh . update (500 , false) ; assert_eq ! (ssthresh . get () , 500) ; assert_eq ! (ssthresh . startup_exit () , Some (expected_startup_exit)) ; } # [test] fn ssthresh_in_slow_start () { let expected_startup_exit = StartupExit :: new (1000 , None , StartupExitReason :: Loss) ; let mut ssthresh : SsThresh = Default :: default () ; ssthresh . update (1000 , false) ; assert_eq ! (ssthresh . get () , 1000) ; assert_eq ! (ssthresh . startup_exit () , Some (expected_startup_exit)) ; ssthresh . update (2000 , true) ; assert_eq ! (ssthresh . get () , 2000) ; assert_eq ! (ssthresh . startup_exit () , Some (expected_startup_exit)) ; ssthresh . update (500 , false) ; assert_eq ! (ssthresh . get () , 500) ; assert_eq ! (ssthresh . startup_exit () , Some (expected_startup_exit)) ; } }
};
}
