// Generated macro for RelocDistance (enum)
macro_rules! Depcrate_machinst_lowerRelocDistance {
() => {
// Module: crate::machinst::lower
// Provides: {"RelocDistance"}
// Dependencies: {}
# [doc = " Notion of \"relocation distance\". This gives an estimate of how far away a symbol will be from a"] # [doc = " reference."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RelocDistance { # [doc = " Target of relocation is \"nearby\". The threshold for this is fuzzy but should be interpreted"] # [doc = " as approximately \"within the compiled output of one module\"; e.g., within AArch64's +/-"] # [doc = " 128MB offset. If unsure, use `Far` instead."] Near , # [doc = " Target of relocation could be anywhere in the address space."] Far , }
};
}
