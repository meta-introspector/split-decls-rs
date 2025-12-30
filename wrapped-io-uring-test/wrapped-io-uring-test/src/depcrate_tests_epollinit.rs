// Generated macro for init (function)
macro_rules! Depcrate_tests_epollinit {
() => {
// Module: crate::tests::epoll
// Provides: {"init"}
// Dependencies: {}
fn init < const NPIPES : usize > () -> anyhow :: Result < (RawFd , [RxTxPipe ; NPIPES] , [:: libc :: epoll_event ; NPIPES]) > { let pipes : [RxTxPipe ; NPIPES] = { let mut pipes : [MaybeUninit < RxTxPipe > ; NPIPES] = [const { MaybeUninit :: uninit () } ; NPIPES] ; for pipe in & mut pipes { let (rx , tx) = std :: io :: pipe () ? ; pipe . write (RxTxPipe { rx , tx }) ; } unsafe { :: core :: mem :: transmute_copy (& pipes) } } ; let epfd = unsafe { :: libc :: epoll_create1 (0) } ; assert ! (epfd >= 0) ; for (pipe , rx_idx) in pipes . iter () . zip (0u64 ..) { let rx = pipe . rx . as_fd () ; let mut events = :: libc :: epoll_event { events : :: libc :: EPOLLIN . cast_unsigned () , u64 : rx_idx , } ; let res = unsafe { :: libc :: epoll_ctl (epfd , :: libc :: EPOLL_CTL_ADD , rx . as_raw_fd () as _ , & mut events ,) } ; assert ! (res >= 0) ; } let events = unsafe { :: core :: mem :: zeroed () } ; Ok ((epfd , pipes , events)) }
};
}
