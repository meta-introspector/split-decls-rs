// Generated macro for impl_128 (impl)
macro_rules! Depcrate_intimpl_128 {
() => {
// Module: crate::int
// Provides: {"impl_128"}
// Dependencies: {}
impl < const LIMBS : usize > AsMut < [Word ; LIMBS] > for Int < LIMBS > { fn as_mut (& mut self) -> & mut [Word ; LIMBS] { self . as_mut_words () } }
};
}
