// Generated macro for PatchRegion (struct)
macro_rules! Depcrate_machinst_bufferPatchRegion {
() => {
// Module: crate::machinst::buffer
// Provides: {"PatchRegion"}
// Dependencies: {}
# [doc = " A region in the [`MachBuffer`] code buffer that can be edited prior to finalization. An example"] # [doc = " of where you might want to use this is for patching instructions that mention constants that"] # [doc = " won't be known until later: [`MachBuffer::start_patchable`] can be used to begin the patchable"] # [doc = " region, instructions can be emitted with placeholder constants, and the [`PatchRegion`] token"] # [doc = " can be produced by [`MachBuffer::end_patchable`]. Once the values of those constants are known,"] # [doc = " the [`PatchRegion::patch`] function can be used to get a mutable buffer to the instruction"] # [doc = " bytes, and the constants uses can be updated directly."] pub struct PatchRegion { range : std :: ops :: Range < usize > , }
};
}
