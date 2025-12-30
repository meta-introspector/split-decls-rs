// Generated macro for MachTextSectionBuilder (struct)
macro_rules! Depcrate_machinst_bufferMachTextSectionBuilder {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachTextSectionBuilder"}
// Dependencies: {}
# [doc = " Implementation of the `TextSectionBuilder` trait backed by `MachBuffer`."] # [doc = ""] # [doc = " Note that `MachBuffer` was primarily written for intra-function references"] # [doc = " of jumps between basic blocks, but it's also quite usable for entire text"] # [doc = " sections and resolving references between functions themselves. This"] # [doc = " builder interprets \"blocks\" as labeled functions for the purposes of"] # [doc = " resolving labels internally in the buffer."] pub struct MachTextSectionBuilder < I : VCodeInst > { buf : MachBuffer < I > , next_func : usize , force_veneers : ForceVeneers , }
};
}
