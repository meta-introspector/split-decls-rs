// Generated macro for MoveWideConst (struct)
macro_rules! Depcrate_isa_aarch64_inst_immsMoveWideConst {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"MoveWideConst"}
// Dependencies: {}
# [doc = " A 16-bit immediate for a MOVZ instruction, with a {0,16,32,48}-bit shift."] # [derive (Clone , Copy , Debug)] pub struct MoveWideConst { # [doc = " The value."] pub bits : u16 , # [doc = " Result is `bits` shifted 16*shift bits to the left."] pub shift : u8 , }
};
}
