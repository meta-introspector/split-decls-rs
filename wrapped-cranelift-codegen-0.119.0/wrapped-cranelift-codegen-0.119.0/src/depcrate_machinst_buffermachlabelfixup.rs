// Generated macro for MachLabelFixup (struct)
macro_rules! Depcrate_machinst_bufferMachLabelFixup {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachLabelFixup"}
// Dependencies: {}
# [doc = " A fixup to perform on the buffer once code is emitted. Fixups always refer"] # [doc = " to labels and patch the code based on label offsets. Hence, they are like"] # [doc = " relocations, but internal to one buffer."] # [derive (Debug)] struct MachLabelFixup < I : VCodeInst > { # [doc = " The label whose offset controls this fixup."] label : MachLabel , # [doc = " The offset to fix up / patch to refer to this label."] offset : CodeOffset , # [doc = " The kind of fixup. This is architecture-specific; each architecture may have,"] # [doc = " e.g., several types of branch instructions, each with differently-sized"] # [doc = " offset fields and different places within the instruction to place the"] # [doc = " bits."] kind : I :: LabelUse , }
};
}
