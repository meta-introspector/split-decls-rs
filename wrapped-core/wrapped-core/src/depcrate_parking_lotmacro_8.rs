// Generated macro for macro_8 (macro)
macro_rules! Depcrate_parking_lotmacro_8 {
() => {
// Module: crate::parking_lot
// Provides: {"macro_8"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (all (target_family = "wasm" , target_os = "unknown" , target_vendor = "unknown"))] { # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] struct TimeoutInstant ; impl TimeoutInstant { fn now () -> TimeoutInstant { TimeoutInstant } } impl core :: ops :: Add < Duration > for TimeoutInstant { type Output = Self ; fn add (self , _rhs : Duration) -> Self :: Output { TimeoutInstant } } } else { use std :: time :: Instant as TimeoutInstant ; } }
};
}
