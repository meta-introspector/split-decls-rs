// Generated macro for impl_242 (impl)
macro_rules! Depcrate_datetimeimpl_242 {
() => {
// Module: crate::datetime
// Provides: {"impl_242"}
// Dependencies: {}
# [cfg (feature = "clock")] impl From < SystemTime > for DateTime < Local > { fn from (t : SystemTime) -> DateTime < Local > { DateTime :: < Utc > :: from (t) . with_timezone (& Local) } }
};
}
