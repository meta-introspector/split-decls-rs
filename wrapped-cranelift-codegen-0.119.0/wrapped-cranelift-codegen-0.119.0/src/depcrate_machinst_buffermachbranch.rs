// Generated macro for MachBranch (struct)
macro_rules! Depcrate_machinst_bufferMachBranch {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachBranch"}
// Dependencies: {}
# [doc = " Record of branch instruction in the buffer, to facilitate editing."] # [derive (Clone , Debug)] struct MachBranch { start : CodeOffset , end : CodeOffset , target : MachLabel , fixup : usize , inverted : Option < SmallVec < [u8 ; 8] > > , # [doc = " All labels pointing to the start of this branch. For correctness, this"] # [doc = " *must* be complete (i.e., must contain all labels whose resolved offsets"] # [doc = " are at the start of this branch): we rely on being able to redirect all"] # [doc = " labels that could jump to this branch before removing it, if it is"] # [doc = " otherwise unreachable."] labels_at_this_branch : SmallVec < [MachLabel ; 4] > , }
};
}
