// Generated macro for impl_892 (impl)
macro_rules! Depcrate_timestampimpl_892 {
() => {
// Module: crate::timestamp
// Provides: {"impl_892"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < std :: time :: SystemTime > for Timestamp { type Error = Error ; # [inline] fn try_from (system_time : std :: time :: SystemTime ,) -> Result < Timestamp , Error > { let unix_epoch = std :: time :: SystemTime :: UNIX_EPOCH ; let dur = SignedDuration :: system_until (unix_epoch , system_time) ? ; Timestamp :: from_duration (dur) } }
};
}
