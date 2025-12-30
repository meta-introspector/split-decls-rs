// Generated macro for test_timeout_submit_args (function)
macro_rules! Depcrate_tests_timeouttest_timeout_submit_args {
() => {
// Module: crate::tests::timeout
// Provides: {"test_timeout_submit_args"}
// Dependencies: {}
pub fn test_timeout_submit_args < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! { test ; ring . params () . is_feature_ext_arg () ; } ; println ! ("test timeout_submit_args") ; let ts = types :: Timespec :: new () . sec (1) ; let args = types :: SubmitArgs :: new () . timespec (& ts) ; let start = Instant :: now () ; match ring . submitter () . submit_with_args (1 , & args) { Ok (_) => panic ! () , Err (ref err) if err . raw_os_error () == Some (libc :: ETIME) => () , Err (err) => return Err (err . into ()) , } assert_eq ! (start . elapsed () . as_secs () , 1) ; assert ! (ring . completion () . next () . is_none ()) ; let nop_e = opcode :: Nop :: new () ; unsafe { ring . submission () . push (& nop_e . build () . user_data (0x1c) . into ()) . expect ("queue is full") ; } let start = Instant :: now () ; ring . submitter () . submit_with_args (1 , & args) ? ; assert_eq ! (start . elapsed () . as_secs () , 0) ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x1c) ; assert_eq ! (cqes [0] . result () , 0) ; Ok (()) }
};
}
