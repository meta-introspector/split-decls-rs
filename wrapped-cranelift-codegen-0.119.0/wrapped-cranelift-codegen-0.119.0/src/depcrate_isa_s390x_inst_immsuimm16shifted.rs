// Generated macro for UImm16Shifted (struct)
macro_rules! Depcrate_isa_s390x_inst_immsUImm16Shifted {
() => {
// Module: crate::isa::s390x::inst::imms
// Provides: {"UImm16Shifted"}
// Dependencies: {}
# [doc = " A 16-bit immediate with a {0,16,32,48}-bit shift."] # [derive (Clone , Copy , Debug)] pub struct UImm16Shifted { # [doc = " The value."] pub bits : u16 , # [doc = " Result is `bits` shifted 16*shift bits to the left."] pub shift : u8 , }
};
}
