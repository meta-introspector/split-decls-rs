// Generated macro for test_remove (function)
macro_rules! Depcrate_tests_epolltest_remove {
() => {
// Module: crate::tests::epoll
// Provides: {"test_remove"}
// Dependencies: {}
pub fn test_remove < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: EpollWait :: CODE) ;) ; println ! ("test epoll_wait_remove") ; const NPIPES : usize = 2 ; let (epfd , mut pipes , mut events) = init :: < NPIPES > () ? ; let sqe = opcode :: EpollWait :: new (types :: Fd (epfd . as_raw_fd ()) , events . as_mut_ptr () . cast () , NPIPES as _ ,) . build () . user_data (REQ_TYPE_EPOLL_WAIT) . into () ; unsafe { ring . submission () . push (& sqe) } ? ; let epfd = unsafe { :: std :: os :: fd :: OwnedFd :: from_raw_fd (epfd) } ; drop (epfd) ; thread :: sleep (Duration :: from_micros (10000)) ; for pipe in & mut pipes { pipe . tx . write_all (DATA) ? ; } ring . submit_and_wait (1) ? ; for cqe in ring . completion () . map (Into :: < cqueue :: Entry > :: into) . take (1) { assert_eq ! (cqe . user_data () , REQ_TYPE_EPOLL_WAIT) ; let err = cqe . result () ; assert ! ([-:: libc :: EAGAIN , -:: libc :: EBADF] . contains (& err)) ; } Ok (()) }
};
}
