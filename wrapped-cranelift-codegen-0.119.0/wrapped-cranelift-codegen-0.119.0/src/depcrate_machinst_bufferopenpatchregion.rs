// Generated macro for OpenPatchRegion (struct)
macro_rules! Depcrate_machinst_bufferOpenPatchRegion {
() => {
// Module: crate::machinst::buffer
// Provides: {"OpenPatchRegion"}
// Dependencies: {}
# [doc = " Represents the beginning of an editable region in the [`MachBuffer`], while code emission is"] # [doc = " still occurring. An [`OpenPatchRegion`] is closed by [`MachBuffer::end_patchable`], consuming"] # [doc = " the [`OpenPatchRegion`] token in the process."] pub struct OpenPatchRegion (usize) ;
};
}
