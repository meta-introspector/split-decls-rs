// Generated macro for LabelUse (enum)
macro_rules! Depcrate_isa_s390x_instLabelUse {
() => {
// Module: crate::isa::s390x::inst
// Provides: {"LabelUse"}
// Dependencies: {}
# [doc = " Different forms of label references for different instruction formats."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum LabelUse { # [allow (dead_code)] # [doc = " RI-format branch.  16-bit signed offset.  PC-relative, offset is imm << 1."] BranchRI , # [doc = " RIL-format branch.  32-bit signed offset.  PC-relative, offset is imm << 1."] BranchRIL , # [doc = " 32-bit PC relative constant offset (from address of constant itself),"] # [doc = " signed. Used in jump tables."] PCRel32 , # [doc = " 32-bit PC relative constant offset (from address of call instruction),"] # [doc = " signed. Offset is imm << 1.  Used for call relocations."] PCRel32Dbl , }
};
}
