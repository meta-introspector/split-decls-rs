// Generated macro for get_untouched_fds (function)
macro_rules! Depcrateget_untouched_fds {
() => {
// Module: crate
// Provides: {"get_untouched_fds"}
// Dependencies: {}
# [cfg (feature = "close-range")] fn get_untouched_fds (except : & [RawFd]) -> Vec < std :: ops :: Range < RawFd > > { if except . is_empty () { return vec ! [0 .. RawFd :: MAX] ; } let mut except = except . to_vec () ; except . sort_unstable () ; let mut ranges = vec ! [] ; if except [0] > 0 { ranges . push (0 .. except [0]) ; } for range in except . windows (2) { if range [0] + 1 == range [1] { continue ; } ranges . push (range [0] + 1 .. range [1]) ; } if except [except . len () - 1] < RawFd :: MAX { ranges . push ((except [except . len () - 1] + 1) .. RawFd :: MAX) ; } ranges }
};
}
