// Generated macro for impl_15 (impl)
macro_rules! Depcrate_control_bitmaskimpl_15 {
() => {
// Module: crate::control::bitmask
// Provides: {"impl_15"}
// Dependencies: {}
impl Iterator for BitMaskIter { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { let bit = self . 0 . lowest_set_bit () ? ; self . 0 = self . 0 . remove_lowest_bit () ; Some (bit) } }
};
}
