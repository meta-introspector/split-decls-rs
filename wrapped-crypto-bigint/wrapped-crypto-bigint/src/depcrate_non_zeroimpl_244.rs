// Generated macro for impl_244 (impl)
macro_rules! Depcrate_non_zeroimpl_244 {
() => {
// Module: crate::non_zero
// Provides: {"impl_244"}
// Dependencies: {}
impl < const LIMBS : usize > From < NonZeroU8 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU8) -> Self { Self :: from_u8 (integer) } }
};
}
