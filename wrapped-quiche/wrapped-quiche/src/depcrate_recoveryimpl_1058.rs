// Generated macro for impl_1058 (impl)
macro_rules! Depcrate_recoveryimpl_1058 {
() => {
// Module: crate::recovery
// Provides: {"impl_1058"}
// Dependencies: {}
impl ReleaseDecision { pub (crate) const EQUAL_THRESHOLD : Duration = Duration :: from_micros (50) ; # [doc = " Get the [`Instant`] the next packet should be released. It will never be"] # [doc = " in the past."] # [allow (dead_code)] # [inline] pub fn time (& self , now : Instant) -> Option < Instant > { match self . time { ReleaseTime :: Immediate => None , ReleaseTime :: At (other) => other . gt (& now) . then_some (other) , } } # [doc = " Can this packet be appended to a previous burst"] # [allow (dead_code)] # [inline] pub fn can_burst (& self) -> bool { self . allow_burst } # [doc = " Check if the two packets can be released at the same time"] # [allow (dead_code)] # [inline] pub fn time_eq (& self , other : & Self , now : Instant) -> bool { let delta = match (self . time (now) , other . time (now)) { (None , None) => Duration :: ZERO , (Some (t) , None) | (None , Some (t)) => t . duration_since (now) , (Some (t1) , Some (t2)) if t1 < t2 => t2 . duration_since (t1) , (Some (t1) , Some (t2)) => t1 . duration_since (t2) , } ; delta <= Self :: EQUAL_THRESHOLD } }
};
}
