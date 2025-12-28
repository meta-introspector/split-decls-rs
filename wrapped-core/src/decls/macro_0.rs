macro_rules! macro_0 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (all (target_family = "wasm" , target_os = "unknown" , target_vendor = "unknown"))] { # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] struct TimeoutInstant ; impl TimeoutInstant { fn now () -> TimeoutInstant { TimeoutInstant } } impl core :: ops :: Add < Duration > for TimeoutInstant { type Output = Self ; fn add (self , _rhs : Duration) -> Self :: Output { TimeoutInstant } } } else { use std :: time :: Instant as TimeoutInstant ; } }
    };
}

macro_0!()