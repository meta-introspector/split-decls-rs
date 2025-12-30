// Generated macro for impl_246 (impl)
macro_rules! Depcrate_non_zeroimpl_246 {
() => {
// Module: crate::non_zero
// Provides: {"impl_246"}
// Dependencies: {}
impl < const LIMBS : usize > From < NonZeroU32 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU32) -> Self { Self :: from_u32 (integer) } }
};
}
