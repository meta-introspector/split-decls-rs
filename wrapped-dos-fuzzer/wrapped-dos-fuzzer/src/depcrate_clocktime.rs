// Generated macro for time (function)
macro_rules! Depcrate_clocktime {
() => {
// Module: crate::clock
// Provides: {"time"}
// Dependencies: {}
fn time (clk_id : libc :: clockid_t) -> Duration { unsafe { let mut timespec = mem :: zeroed () ; if libc :: clock_gettime (clk_id , & mut timespec) != 0 { let err : Result < () , _ > = Err (Error :: last_os_error ()) ; err . expect (& format ! ("Error creating clock with clk_id {}" , clk_id)) ; } Duration :: new (timespec . tv_sec as u64 , timespec . tv_nsec as u32) } }
};
}
