// Generated macro for test_ready (function)
macro_rules! Depcrate_tests_epolltest_ready {
() => {
// Module: crate::tests::epoll
// Provides: {"test_ready"}
// Dependencies: {}
pub fn test_ready < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: EpollWait :: CODE) ;) ; println ! ("test epoll_wait_ready") ; const NPIPES : usize = 2 ; let (epfd , mut pipes , mut events) = init :: < NPIPES > () ? ; for pipe in & mut pipes { pipe . tx . write_all (DATA) ? ; } let sqe = opcode :: EpollWait :: new (types :: Fd (epfd . as_raw_fd ()) , events . as_mut_ptr () . cast () , NPIPES as _ ,) . build () . user_data (REQ_TYPE_EPOLL_WAIT) . into () ; unsafe { ring . submission () . push (& sqe) } ? ; ring . submit_and_wait (1) ? ; let cqe = ring . completion () . map (Into :: < cqueue :: Entry > :: into) . next () . unwrap () ; assert_eq ! (cqe . user_data () , REQ_TYPE_EPOLL_WAIT) ; assert_eq ! (cqe . result () , 2) ; for rx_idx in events { let mut tmp = [0u8 ; 16] ; let len = pipes [rx_idx . u64 as usize] . rx . read (& mut tmp) ? ; assert_eq ! (len , DATA . len ()) ; assert_eq ! (& tmp [.. len] , DATA) ; } Ok (()) }
};
}
