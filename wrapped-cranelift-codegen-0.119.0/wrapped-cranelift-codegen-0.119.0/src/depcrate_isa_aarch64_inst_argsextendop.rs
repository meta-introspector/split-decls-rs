// Generated macro for ExtendOp (enum)
macro_rules! Depcrate_isa_aarch64_inst_argsExtendOp {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"ExtendOp"}
// Dependencies: {}
# [doc = " An extend operator for a register."] # [derive (Clone , Copy , Debug)] # [repr (u8)] pub enum ExtendOp { # [doc = " Unsigned extend byte."] UXTB = 0b000 , # [doc = " Unsigned extend halfword."] UXTH = 0b001 , # [doc = " Unsigned extend word."] UXTW = 0b010 , # [doc = " Unsigned extend doubleword."] UXTX = 0b011 , # [doc = " Signed extend byte."] SXTB = 0b100 , # [doc = " Signed extend halfword."] SXTH = 0b101 , # [doc = " Signed extend word."] SXTW = 0b110 , # [doc = " Signed extend doubleword."] SXTX = 0b111 , }
};
}
