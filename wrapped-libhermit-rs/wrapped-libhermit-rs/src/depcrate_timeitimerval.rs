// Generated macro for itimerval (struct)
macro_rules! Depcrate_timeitimerval {
() => {
// Module: crate::time
// Provides: {"itimerval"}
// Dependencies: {}
# [doc = " Represent the timer interval in seconds and microseconds"] # [derive (Copy , Clone , Debug)] # [repr (C)] pub struct itimerval { pub it_interval : timeval , pub it_value : timeval , }
};
}
