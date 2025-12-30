// Generated macro for FcmpImm (enum)
macro_rules! Depcrate_isa_x64_inst_argsFcmpImm {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"FcmpImm"}
// Dependencies: {}
# [doc = " Encode the ways that floats can be compared. This is used in float comparisons such as `cmpps`,"] # [doc = " e.g.; it is distinguished from other float comparisons (e.g. `ucomiss`) in that those use EFLAGS"] # [doc = " whereas [FcmpImm] is used as an immediate."] # [derive (Clone , Copy)] pub enum FcmpImm { # [doc = " Equal comparison."] Equal = 0x00 , # [doc = " Less than comparison."] LessThan = 0x01 , # [doc = " Less than or equal comparison."] LessThanOrEqual = 0x02 , # [doc = " Unordered."] Unordered = 0x03 , # [doc = " Not equal comparison."] NotEqual = 0x04 , # [doc = " Unordered of greater than or equal comparison."] UnorderedOrGreaterThanOrEqual = 0x05 , # [doc = " Unordered or greater than comparison."] UnorderedOrGreaterThan = 0x06 , # [doc = " Ordered."] Ordered = 0x07 , }
};
}
