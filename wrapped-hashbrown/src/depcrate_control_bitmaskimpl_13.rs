// Generated macro for impl_13 (impl)
macro_rules! Depcrate_control_bitmaskimpl_13 {
() => {
// Module: crate::control::bitmask
// Provides: {"impl_13"}
// Dependencies: {}
impl IntoIterator for BitMask { type Item = usize ; type IntoIter = BitMaskIter ; # [inline] fn into_iter (self) -> BitMaskIter { BitMaskIter (BitMask (self . 0 & BITMASK_ITER_MASK)) } }
};
}
