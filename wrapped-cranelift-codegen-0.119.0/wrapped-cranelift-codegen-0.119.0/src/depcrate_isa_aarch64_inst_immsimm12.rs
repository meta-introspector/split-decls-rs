// Generated macro for Imm12 (struct)
macro_rules! Depcrate_isa_aarch64_inst_immsImm12 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"Imm12"}
// Dependencies: {}
# [doc = " A shifted immediate value in 'imm12' format: supports 12 bits, shifted"] # [doc = " left by 0 or 12 places."] # [derive (Copy , Clone , Debug)] pub struct Imm12 { # [doc = " The immediate bits."] pub bits : u16 , # [doc = " Whether the immediate bits are shifted left by 12 or not."] pub shift12 : bool , }
};
}
