// Generated macro for test_async_cancel_fd_all (function)
macro_rules! Depcrate_tests_canceltest_async_cancel_fd_all {
() => {
// Module: crate::tests::cancel
// Provides: {"test_async_cancel_fd_all"}
// Dependencies: {}
pub fn test_async_cancel_fd_all < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: PollAdd :: CODE) ; test . probe . is_supported (opcode :: AsyncCancel2 :: CODE) ; test . probe . is_supported (opcode :: Socket :: CODE) ;) ; println ! ("test async_cancel_fd_all") ; let _fd = create_dummy_fd () ? ; let fd = types :: Fd (_fd . as_raw_fd ()) ; let poll_e = opcode :: PollAdd :: new (fd , libc :: POLLIN as _) . build () ; let builder = CancelBuilder :: fd (fd) . all () ; let cancel_e = opcode :: AsyncCancel2 :: new (builder) . build () ; let entries = [poll_e . clone () . user_data (2003) . into () , poll_e . user_data (2004) . into () , cancel_e . user_data (2005) . into () ,] ; for sqe in & entries { unsafe { ring . submission () . push (sqe) . expect ("queue is full") ; } } ring . submit_and_wait (entries . len ()) ? ; let mut cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; cqes . sort_unstable_by_key (cqueue :: Entry :: user_data) ; assert_eq ! (cqes . len () , entries . len ()) ; assert_eq ! (cqes [0] . user_data () , 2003) ; assert_eq ! (cqes [1] . user_data () , 2004) ; assert_eq ! (cqes [2] . user_data () , 2005) ; assert_eq ! (cqes [0] . result () , - libc :: ECANCELED) ; assert_eq ! (cqes [1] . result () , - libc :: ECANCELED) ; assert_eq ! (cqes [2] . result () , 2) ; Ok (()) }
};
}
