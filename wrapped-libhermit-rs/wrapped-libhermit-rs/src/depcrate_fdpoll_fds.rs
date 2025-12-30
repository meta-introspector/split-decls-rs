// Generated macro for poll_fds (function)
macro_rules! Depcrate_fdpoll_fds {
() => {
// Module: crate::fd
// Provides: {"poll_fds"}
// Dependencies: {}
async fn poll_fds (fds : & mut [PollFd]) -> io :: Result < u64 > { future :: poll_fn (| cx | { let mut counter : u64 = 0 ; for i in & mut * fds { let fd = i . fd ; i . revents = PollEvent :: empty () ; if let Ok (obj) = core_scheduler () . get_object (fd) { let mut pinned = core :: pin :: pin ! (async { obj . read () . await . poll (i . events) . await }) ; if let Ready (Ok (e)) = pinned . as_mut () . poll (cx) && ! e . is_empty () { counter += 1 ; i . revents = e ; } } } if counter > 0 { Ready (Ok (counter)) } else { Pending } }) . await }
};
}
