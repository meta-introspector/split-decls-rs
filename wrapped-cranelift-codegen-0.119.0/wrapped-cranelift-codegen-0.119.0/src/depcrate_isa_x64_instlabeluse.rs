// Generated macro for LabelUse (enum)
macro_rules! Depcrate_isa_x64_instLabelUse {
() => {
// Module: crate::isa::x64::inst
// Provides: {"LabelUse"}
// Dependencies: {}
# [doc = " A label-use (internal relocation) in generated code."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum LabelUse { # [doc = " A 32-bit offset from location of relocation itself, added to the existing value at that"] # [doc = " location. Used for control flow instructions which consider an offset from the start of the"] # [doc = " next instruction (so the size of the payload -- 4 bytes -- is subtracted from the payload)."] JmpRel32 , # [doc = " A 32-bit offset from location of relocation itself, added to the existing value at that"] # [doc = " location."] PCRel32 , }
};
}
