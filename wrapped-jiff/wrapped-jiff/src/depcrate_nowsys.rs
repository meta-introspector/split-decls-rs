// Generated macro for sys (module)
macro_rules! Depcrate_nowsys {
() => {
// Module: crate::now
// Provides: {"sys"}
// Dependencies: {}
# [cfg (all (feature = "js" , any (target_arch = "wasm32" , target_arch = "wasm64") , target_os = "unknown"))] mod sys { pub (crate) fn system_time () -> std :: time :: SystemTime { use std :: time :: { Duration , SystemTime } ; # [cfg (not (feature = "std"))] use crate :: util :: libm :: Float ; let millis = js_sys :: Date :: new_0 () . get_time () ; let sign = millis . signum () ; let millis = millis . abs () as u64 ; let duration = Duration :: from_millis (millis) ; let result = if sign >= 0.0 { SystemTime :: UNIX_EPOCH . checked_add (duration) } else { SystemTime :: UNIX_EPOCH . checked_sub (duration) } ; let Some (timestamp) = result else { panic ! ("failed to get current time: \
             subtracting {duration:?} from Unix epoch overflowed") } ; timestamp } # [cfg (any (feature = "tz-system" , feature = "tzdb-zoneinfo" , feature = "tzdb-concatenated"))] pub (crate) fn monotonic_time () -> Option < std :: time :: Instant > { None } }
};
}
