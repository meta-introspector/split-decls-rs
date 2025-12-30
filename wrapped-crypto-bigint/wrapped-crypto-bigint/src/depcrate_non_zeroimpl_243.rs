// Generated macro for impl_243 (impl)
macro_rules! Depcrate_non_zeroimpl_243 {
() => {
// Module: crate::non_zero
// Provides: {"impl_243"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] impl From < NonZeroU64 > for NonZero < Limb > { fn from (integer : NonZeroU64) -> Self { Self :: from_u64 (integer) } }
};
}
