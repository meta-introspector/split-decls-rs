// Generated macro for impl_271 (impl)
macro_rules! Depcrate_machinst_bufferimpl_271 {
() => {
// Module: crate::machinst::buffer
// Provides: {"impl_271"}
// Dependencies: {}
impl < I : VCodeInst > Extend < u8 > for MachBuffer < I > { fn extend < T : IntoIterator < Item = u8 > > (& mut self , iter : T) { for b in iter { self . put1 (b) ; } } }
};
}
