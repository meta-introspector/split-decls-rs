// Generated macro for InstructionSet (enum)
macro_rules! Depcrate_isa_s390x_instInstructionSet {
() => {
// Module: crate::isa::s390x::inst
// Provides: {"InstructionSet"}
// Dependencies: {}
# [doc = " Supported instruction sets"] # [allow (non_camel_case_types)] # [derive (Debug)] pub (crate) enum InstructionSet { # [doc = " Baseline ISA for cranelift is z14."] Base , # [doc = " Miscellaneous-Instruction-Extensions Facility 2 (z15)"] MIE2 , # [doc = " Vector-Enhancements Facility 2 (z15)"] VXRS_EXT2 , }
};
}
