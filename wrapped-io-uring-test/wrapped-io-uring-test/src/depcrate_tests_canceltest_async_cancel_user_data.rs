// Generated macro for test_async_cancel_user_data (function)
macro_rules! Depcrate_tests_canceltest_async_cancel_user_data {
() => {
// Module: crate::tests::cancel
// Provides: {"test_async_cancel_user_data"}
// Dependencies: {}
pub fn test_async_cancel_user_data < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Timeout :: CODE) ; test . probe . is_supported (opcode :: AsyncCancel2 :: CODE) ;) ; println ! ("test async_cancel_user_data") ; let ts = types :: Timespec :: new () . sec (1) ; let timeout_e = opcode :: Timeout :: new (& ts) . build () ; let builder = CancelBuilder :: user_data (2003) ; let cancel_e = opcode :: AsyncCancel2 :: new (builder) . build () ; let entries = [timeout_e . user_data (2003) . into () , cancel_e . user_data (2004) . into () ,] ; for sqe in & entries { unsafe { ring . submission () . push (sqe) . expect ("queue is full") ; } } ring . submit_and_wait (entries . len ()) ? ; let mut cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; cqes . sort_unstable_by_key (cqueue :: Entry :: user_data) ; assert_eq ! (cqes . len () , entries . len ()) ; assert_eq ! (cqes [0] . user_data () , 2003) ; assert_eq ! (cqes [1] . user_data () , 2004) ; assert_eq ! (cqes [0] . result () , - libc :: ECANCELED) ; assert_eq ! (cqes [1] . result () , 0) ; Ok (()) }
};
}
