// Generated macro for impl_276 (impl)
macro_rules! Depcrate_machinst_bufferimpl_276 {
() => {
// Module: crate::machinst::buffer
// Provides: {"impl_276"}
// Dependencies: {}
impl < I : VCodeInst > MachLabelFixup < I > { fn deadline (& self) -> CodeOffset { self . offset . saturating_add (self . kind . max_pos_range ()) } }
};
}
