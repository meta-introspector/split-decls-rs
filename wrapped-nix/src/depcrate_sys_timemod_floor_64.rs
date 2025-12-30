// Generated macro for mod_floor_64 (function)
macro_rules! Depcrate_sys_timemod_floor_64 {
() => {
// Module: crate::sys::time
// Provides: {"mod_floor_64"}
// Dependencies: {}
# [inline] fn mod_floor_64 (this : i64 , other : i64) -> i64 { match this % other { r if (r > 0 && other < 0) || (r < 0 && other > 0) => r + other , r => r , } }
};
}
