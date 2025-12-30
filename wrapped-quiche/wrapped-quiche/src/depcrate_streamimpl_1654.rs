// Generated macro for impl_1654 (impl)
macro_rules! Depcrate_streamimpl_1654 {
() => {
// Module: crate::stream
// Provides: {"impl_1654"}
// Dependencies: {}
impl Iterator for StreamIter { type Item = u64 ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let v = self . streams . get (self . index) ? ; self . index += 1 ; Some (* v) } }
};
}
