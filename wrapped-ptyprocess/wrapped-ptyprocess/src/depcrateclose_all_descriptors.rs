// Generated macro for close_all_descriptors (function)
macro_rules! Depcrateclose_all_descriptors {
() => {
// Module: crate
// Provides: {"close_all_descriptors"}
// Dependencies: {}
# [cfg (feature = "close-range")] fn close_all_descriptors (except : & [RawFd]) -> Result < () > { if except . is_empty () { return Ok (()) ; } let fds = get_untouched_fds (except) ; for range in fds { match range . len () { 0 => unreachable ! ("must never happen") , 1 => _ = close (range . start) , _ => { let first = range . start as std :: ffi :: c_uint ; let last = (range . end as std :: ffi :: c_uint) . saturating_sub (1) ; let _ = unsafe { nix :: libc :: close_range (first , last , 0) } ; } } } Ok (()) }
};
}
