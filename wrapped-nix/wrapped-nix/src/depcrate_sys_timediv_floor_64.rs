// Generated macro for div_floor_64 (function)
macro_rules! Depcrate_sys_timediv_floor_64 {
() => {
// Module: crate::sys::time
// Provides: {"div_floor_64"}
// Dependencies: {}
# [inline] fn div_floor_64 (this : i64 , other : i64) -> i64 { match div_rem_64 (this , other) { (d , r) if (r > 0 && other < 0) || (r < 0 && other > 0) => d - 1 , (d , _) => d , } }
};
}
