// Generated macro for test_delete (function)
macro_rules! Depcrate_tests_epolltest_delete {
() => {
// Module: crate::tests::epoll
// Provides: {"test_delete"}
// Dependencies: {}
pub fn test_delete < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: EpollWait :: CODE) ;) ; println ! ("test epoll_wait_delete") ; const NPIPES : usize = 2 ; let (epfd , mut pipes , mut events) = init :: < NPIPES > () ? ; let sqe = opcode :: EpollWait :: new (types :: Fd (epfd . as_raw_fd ()) , events . as_mut_ptr () . cast () , NPIPES as _ ,) . build () . user_data (REQ_TYPE_EPOLL_WAIT) . into () ; unsafe { ring . submission () . push (& sqe) } ? ; { let mut event = :: libc :: epoll_event { events : :: libc :: EPOLLIN . cast_unsigned () , u64 : 0 , } ; let res = unsafe { :: libc :: epoll_ctl (epfd , :: libc :: EPOLL_CTL_DEL , pipes [0] . rx . as_raw_fd () , & mut event ,) } ; assert ! (res >= 0) ; } for pipe in & mut pipes { pipe . tx . write_all (DATA) ? ; } ring . submit_and_wait (1) ? ; for cqe in ring . completion () . map (Into :: < cqueue :: Entry > :: into) . take (1) { assert_eq ! (cqe . user_data () , REQ_TYPE_EPOLL_WAIT) ; assert ! (cqe . result () == 1) ; } for pipe in & mut pipes { let mut tmp = [0u8 ; 16] ; let len = pipe . rx . read (& mut tmp) ? ; assert_eq ! (len , DATA . len ()) ; assert_eq ! (& tmp [.. len] , DATA) ; } { let mut event = :: libc :: epoll_event { events : :: libc :: EPOLLIN . cast_unsigned () , u64 : 0 , } ; unsafe { :: libc :: epoll_ctl (epfd , :: libc :: EPOLL_CTL_ADD , pipes [0] . rx . as_raw_fd () , & mut event ,) } ; } Ok (()) }
};
}
