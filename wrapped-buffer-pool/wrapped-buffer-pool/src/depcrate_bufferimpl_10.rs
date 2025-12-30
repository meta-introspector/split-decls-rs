// Generated macro for impl_10 (impl)
macro_rules! Depcrate_bufferimpl_10 {
() => {
// Module: crate::buffer
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a > Extend < & 'a u8 > for ConsumeBuffer { fn extend < T : IntoIterator < Item = & 'a u8 > > (& mut self , iter : T) { self . inner . extend (iter) } }
};
}
