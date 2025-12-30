// Generated macro for SImm7Scaled (struct)
macro_rules! Depcrate_isa_aarch64_inst_immsSImm7Scaled {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"SImm7Scaled"}
// Dependencies: {}
# [doc = " A signed, scaled 7-bit offset."] # [derive (Clone , Copy , Debug)] pub struct SImm7Scaled { # [doc = " The value."] pub value : i16 , # [doc = " multiplied by the size of this type"] pub scale_ty : Type , }
};
}
