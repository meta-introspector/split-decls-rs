// Generated macro for impl_77 (impl)
macro_rules! Depcrate_datetimeimpl_77 {
() => {
// Module: crate::datetime
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < SystemTime > for DateTime { type Error = Error ; fn try_from (time : SystemTime) -> Result < DateTime > { DateTime :: from_system_time (time) } }
};
}
