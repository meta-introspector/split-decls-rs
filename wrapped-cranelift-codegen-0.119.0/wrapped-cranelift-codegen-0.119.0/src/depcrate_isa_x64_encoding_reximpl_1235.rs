// Generated macro for impl_1235 (impl)
macro_rules! Depcrate_isa_x64_encoding_reximpl_1235 {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"impl_1235"}
// Dependencies: {}
impl OpcodeMap { # [doc = " Normally the opcode map is specified as bytes in the instruction, but some x64 encoding"] # [doc = " formats pack this information as bits in a prefix (e.g. VEX / EVEX)."] pub (crate) fn bits (& self) -> u8 { match self { OpcodeMap :: None => 0b00 , OpcodeMap :: _0F => 0b01 , OpcodeMap :: _0F38 => 0b10 , OpcodeMap :: _0F3A => 0b11 , } } }
};
}
