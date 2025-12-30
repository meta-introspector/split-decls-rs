// Generated macro for impl_428 (impl)
macro_rules! Depcrate_uintimpl_428 {
() => {
// Module: crate::uint
// Provides: {"impl_428"}
// Dependencies: {}
impl < const LIMBS : usize > AsMut < [Word ; LIMBS] > for Uint < LIMBS > { fn as_mut (& mut self) -> & mut [Word ; LIMBS] { self . as_mut_words () } }
};
}
