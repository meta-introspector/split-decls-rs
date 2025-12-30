// Generated macro for impl_258 (impl)
macro_rules! Depcrate_machinst_bufferimpl_258 {
() => {
// Module: crate::machinst::buffer
// Provides: {"impl_258"}
// Dependencies: {}
impl MachBufferFinalized < Stencil > { # [doc = " Get a finalized machine buffer by applying the function's base source location."] pub fn apply_base_srcloc (self , base_srcloc : SourceLoc) -> MachBufferFinalized < Final > { MachBufferFinalized { data : self . data , relocs : self . relocs , traps : self . traps , call_sites : self . call_sites , srclocs : self . srclocs . into_iter () . map (| srcloc | srcloc . apply_base_srcloc (base_srcloc)) . collect () , user_stack_maps : self . user_stack_maps , unwind_info : self . unwind_info , alignment : self . alignment , } } }
};
}
