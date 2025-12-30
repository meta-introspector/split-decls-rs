// Generated macro for PerfStatsLevel (enum)
macro_rules! Depcrate_perfPerfStatsLevel {
() => {
// Module: crate::perf
// Provides: {"PerfStatsLevel"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq)] # [repr (i32)] pub enum PerfStatsLevel { # [doc = " Unknown settings"] Uninitialized = 0 , # [doc = " Disable perf stats"] Disable , # [doc = " Enables only count stats"] EnableCount , # [doc = " Count stats and enable time stats except for mutexes"] EnableTimeExceptForMutex , # [doc = " Other than time, also measure CPU time counters. Still don't measure"] # [doc = " time (neither wall time nor CPU time) for mutexes"] EnableTimeAndCPUTimeExceptForMutex , # [doc = " Enables count and time stats"] EnableTime , # [doc = " N.B must always be the last value!"] OutOfBound , }
};
}
