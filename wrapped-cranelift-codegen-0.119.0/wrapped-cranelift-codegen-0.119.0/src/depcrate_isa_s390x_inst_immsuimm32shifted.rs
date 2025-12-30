// Generated macro for UImm32Shifted (struct)
macro_rules! Depcrate_isa_s390x_inst_immsUImm32Shifted {
() => {
// Module: crate::isa::s390x::inst::imms
// Provides: {"UImm32Shifted"}
// Dependencies: {}
# [doc = " A 32-bit immediate with a {0,32}-bit shift."] # [derive (Clone , Copy , Debug)] pub struct UImm32Shifted { # [doc = " The value."] pub bits : u32 , # [doc = " Result is `bits` shifted 32*shift bits to the left."] pub shift : u8 , }
};
}
