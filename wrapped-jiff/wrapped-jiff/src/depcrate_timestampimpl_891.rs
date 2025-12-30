// Generated macro for impl_891 (impl)
macro_rules! Depcrate_timestampimpl_891 {
() => {
// Module: crate::timestamp
// Provides: {"impl_891"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < Timestamp > for std :: time :: SystemTime { # [inline] fn from (time : Timestamp) -> std :: time :: SystemTime { let unix_epoch = std :: time :: SystemTime :: UNIX_EPOCH ; let sdur = time . as_duration () ; let dur = sdur . unsigned_abs () ; if sdur . is_negative () { unix_epoch . checked_sub (dur) . expect ("duration too big (negative)") } else { unix_epoch . checked_add (dur) . expect ("duration too big (positive)") } } }
};
}
