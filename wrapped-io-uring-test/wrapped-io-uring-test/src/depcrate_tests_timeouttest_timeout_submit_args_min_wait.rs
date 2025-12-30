// Generated macro for test_timeout_submit_args_min_wait (function)
macro_rules! Depcrate_tests_timeouttest_timeout_submit_args_min_wait {
() => {
// Module: crate::tests::timeout
// Provides: {"test_timeout_submit_args_min_wait"}
// Dependencies: {}
pub fn test_timeout_submit_args_min_wait < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! { test ; ring . params () . is_feature_ext_arg () ; ring . params () . is_feature_min_timeout () ; } ; println ! ("test timeout_submit_args_min_wait") ; let ts = types :: Timespec :: new () . sec (2) ; let args = types :: SubmitArgs :: new () . timespec (& ts) . min_wait_usec (1_000_000) ; let start = Instant :: now () ; match ring . submitter () . submit_with_args (2 , & args) { Ok (_) => panic ! () , Err (ref err) if err . raw_os_error () == Some (libc :: ETIME) => () , Err (err) => return Err (err . into ()) , } assert_eq ! (start . elapsed () . as_secs () , 2) ; assert ! (ring . completion () . next () . is_none ()) ; let nop_e = opcode :: Nop :: new () ; unsafe { ring . submission () . push (& nop_e . build () . user_data (0x1d) . into ()) . expect ("queue is full") ; } let start = Instant :: now () ; ring . submitter () . submit_with_args (2 , & args) ? ; assert_eq ! (start . elapsed () . as_secs () , 1) ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x1d) ; assert_eq ! (cqes [0] . result () , 0) ; Ok (()) }
};
}
