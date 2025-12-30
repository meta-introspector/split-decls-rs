// Generated macro for impl_247 (impl)
macro_rules! Depcrate_non_zeroimpl_247 {
() => {
// Module: crate::non_zero
// Provides: {"impl_247"}
// Dependencies: {}
impl < const LIMBS : usize > From < NonZeroU64 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU64) -> Self { Self :: from_u64 (integer) } }
};
}
