// Generated macro for LabelUse (enum)
macro_rules! Depcrate_isa_pulley_shared_instLabelUse {
() => {
// Module: crate::isa::pulley_shared::inst
// Provides: {"LabelUse"}
// Dependencies: {}
# [doc = " Different forms of label references for different instruction formats."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum LabelUse { # [doc = " A PC-relative `jump`/`call`/etc... instruction with an `i32` relative"] # [doc = " target. The payload value is an addend that describes the positive"] # [doc = " offset from the start of the instruction to the offset being relocated."] Jump (u32) , }
};
}
