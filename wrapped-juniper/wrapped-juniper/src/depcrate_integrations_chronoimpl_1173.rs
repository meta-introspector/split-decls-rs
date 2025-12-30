// Generated macro for impl_1173 (impl)
macro_rules! Depcrate_integrations_chronoimpl_1173 {
() => {
// Module: crate::integrations::chrono
// Provides: {"impl_1173"}
// Dependencies: {}
# [cfg (feature = "chrono-tz")] impl FromFixedOffset for chrono_tz :: Tz { fn from_fixed_offset (dt : DateTime < FixedOffset >) -> DateTime < Self > { dt . with_timezone (& chrono_tz :: UTC) } }
};
}
