// Generated macro for ImmLogic (struct)
macro_rules! Depcrate_isa_aarch64_inst_immsImmLogic {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"ImmLogic"}
// Dependencies: {}
# [doc = " An immediate for logical instructions."] # [derive (Copy , Clone , Debug , PartialEq)] pub struct ImmLogic { # [doc = " The actual value."] value : u64 , # [doc = " `N` flag."] pub n : bool , # [doc = " `S` field: element size and element bits."] pub r : u8 , # [doc = " `R` field: rotate amount."] pub s : u8 , # [doc = " Was this constructed for a 32-bit or 64-bit instruction?"] pub size : OperandSize , }
};
}
