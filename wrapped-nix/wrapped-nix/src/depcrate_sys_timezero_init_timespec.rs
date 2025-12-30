// Generated macro for zero_init_timespec (function)
macro_rules! Depcrate_sys_timezero_init_timespec {
() => {
// Module: crate::sys::time
// Provides: {"zero_init_timespec"}
// Dependencies: {}
const fn zero_init_timespec () -> timespec { unsafe { std :: mem :: transmute ([0u8 ; std :: mem :: size_of :: < timespec > ()]) } }
};
}
