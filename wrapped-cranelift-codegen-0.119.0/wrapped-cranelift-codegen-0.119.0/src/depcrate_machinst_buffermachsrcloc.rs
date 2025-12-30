// Generated macro for MachSrcLoc (struct)
macro_rules! Depcrate_machinst_bufferMachSrcLoc {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachSrcLoc"}
// Dependencies: {}
# [doc = " A source-location mapping resulting from a compilation."] # [derive (PartialEq , Debug , Clone)] # [cfg_attr (feature = "enable-serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub struct MachSrcLoc < T : CompilePhase > { # [doc = " The start of the region of code corresponding to a source location."] # [doc = " This is relative to the start of the function, not to the start of the"] # [doc = " section."] pub start : CodeOffset , # [doc = " The end of the region of code corresponding to a source location."] # [doc = " This is relative to the start of the section, not to the start of the"] # [doc = " section."] pub end : CodeOffset , # [doc = " The source location."] pub loc : T :: SourceLocType , }
};
}
