// Generated macro for impl_248 (impl)
macro_rules! Depcrate_non_zeroimpl_248 {
() => {
// Module: crate::non_zero
// Provides: {"impl_248"}
// Dependencies: {}
impl < const LIMBS : usize > From < NonZeroU128 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU128) -> Self { Self :: from_u128 (integer) } }
};
}
