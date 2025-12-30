// Generated macro for test_not_ready (function)
macro_rules! Depcrate_tests_epolltest_not_ready {
() => {
// Module: crate::tests::epoll
// Provides: {"test_not_ready"}
// Dependencies: {}
pub fn test_not_ready < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: EpollWait :: CODE) ;) ; println ! ("test epoll_wait_not_ready") ; const NPIPES : usize = 2 ; let (epfd , mut pipes , mut events) = init :: < NPIPES > () ? ; let sqe = opcode :: EpollWait :: new (types :: Fd (epfd . as_raw_fd ()) , events . as_mut_ptr () . cast () , NPIPES as _ ,) . build () . user_data (REQ_TYPE_EPOLL_WAIT) . into () ; unsafe { ring . submission () . push (& sqe) } ? ; for pipe in & mut pipes { thread :: sleep (Duration :: from_micros (10000)) ; pipe . tx . write_all (DATA) ? ; } let mut nr = 0 ; ring . submit_and_wait (1) ? ; for cqe in ring . completion () . map (Into :: < cqueue :: Entry > :: into) . take (1) { assert_eq ! (cqe . user_data () , REQ_TYPE_EPOLL_WAIT) ; assert ! (0 <= cqe . result () && cqe . result () <= 2) ; nr = cqe . result () ; } for rx_idx in events . iter () . take (nr as _) { let mut tmp = [0u8 ; 16] ; let len = pipes [rx_idx . u64 as usize] . rx . read (& mut tmp) ? ; assert_eq ! (len , DATA . len ()) ; assert_eq ! (& tmp [.. len] , DATA) ; } Ok (()) }
};
}
