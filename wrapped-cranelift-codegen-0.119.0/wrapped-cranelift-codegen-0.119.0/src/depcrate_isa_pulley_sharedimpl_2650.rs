// Generated macro for impl_2650 (impl)
macro_rules! Depcrate_isa_pulley_sharedimpl_2650 {
() => {
// Module: crate::isa::pulley_shared
// Provides: {"impl_2650"}
// Dependencies: {}
impl PointerWidth { pub fn bits (self) -> u8 { match self { PointerWidth :: PointerWidth32 => 32 , PointerWidth :: PointerWidth64 => 64 , } } pub fn bytes (self) -> u8 { self . bits () / 8 } }
};
}
