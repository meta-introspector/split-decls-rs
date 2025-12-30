// Generated macro for impl_245 (impl)
macro_rules! Depcrate_non_zeroimpl_245 {
() => {
// Module: crate::non_zero
// Provides: {"impl_245"}
// Dependencies: {}
impl < const LIMBS : usize > From < NonZeroU16 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU16) -> Self { Self :: from_u16 (integer) } }
};
}
