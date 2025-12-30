// Generated macro for impl_123 (impl)
macro_rules! Depcrate_utilsimpl_123 {
() => {
// Module: crate::utils
// Provides: {"impl_123"}
// Dependencies: {}
impl Iterator for BitsIter { type Item = u16 ; fn next (& mut self) -> Option < Self :: Item > { if self . 0 == 0 { return None ; } let bit = self . 0 . trailing_zeros () ; self . 0 ^= (1 << bit) as u16 ; Some (bit as u16) } }
};
}
