// Generated macro for impl_292 (impl)
macro_rules! Depcrate_machinst_bufferimpl_292 {
() => {
// Module: crate::machinst::buffer
// Provides: {"impl_292"}
// Dependencies: {}
impl MachSrcLoc < Stencil > { fn apply_base_srcloc (self , base_srcloc : SourceLoc) -> MachSrcLoc < Final > { MachSrcLoc { start : self . start , end : self . end , loc : self . loc . expand (base_srcloc) , } } }
};
}
