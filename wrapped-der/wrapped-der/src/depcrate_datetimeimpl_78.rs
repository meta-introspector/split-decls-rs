// Generated macro for impl_78 (impl)
macro_rules! Depcrate_datetimeimpl_78 {
() => {
// Module: crate::datetime
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < & SystemTime > for DateTime { type Error = Error ; fn try_from (time : & SystemTime) -> Result < DateTime > { DateTime :: from_system_time (* time) } }
};
}
