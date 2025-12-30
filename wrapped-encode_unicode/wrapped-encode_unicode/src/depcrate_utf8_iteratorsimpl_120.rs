// Generated macro for impl_120 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_120 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_120"}
// Dependencies: {}
impl Iterator for Utf8Iterator { type Item = u8 ; fn next (& mut self) -> Option < u8 > { let next = self . 0 as u8 ; if next == 0xff { None } else { self . 0 = (self . 0 >> 8) | 0xff_00_00_00 ; Some (next) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
