// Generated macro for check_ready_now (function)
macro_rules! Depcrate_connectioncheck_ready_now {
() => {
// Module: crate::connection
// Provides: {"check_ready_now"}
// Dependencies: {}
# [doc = " Call when a read/write would block. Returns Ok(true) if we're ready to go again right now"] fn check_ready_now < 'a > (guard : & mut task :: Poll < AsyncFdReadyGuard < 'a , RawFd > > , poll_ready : impl FnOnce () -> task :: Poll < std :: io :: Result < AsyncFdReadyGuard < 'a , RawFd > > > ,) -> std :: io :: Result < bool > { if let task :: Poll :: Ready (g) = guard { g . clear_ready () ; } let ready_now = poll_ready () ? ; let try_again = ready_now . is_ready () ; * guard = ready_now ; Ok (try_again) }
};
}
