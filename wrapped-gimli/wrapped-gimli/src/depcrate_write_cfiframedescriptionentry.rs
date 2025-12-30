// Generated macro for FrameDescriptionEntry (struct)
macro_rules! Depcrate_write_cfiFrameDescriptionEntry {
() => {
// Module: crate::write::cfi
// Provides: {"FrameDescriptionEntry"}
// Dependencies: {}
# [doc = " A frame description entry. There should be one FDE per function."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct FrameDescriptionEntry { # [doc = " The initial address of the function."] address : Address , # [doc = " The length in bytes of the function."] length : u32 , # [doc = " The address of the LSDA."] pub lsda : Option < Address > , # [doc = " The instructions for this function, ordered by offset."] instructions : Vec < (u32 , CallFrameInstruction) > , }
};
}
