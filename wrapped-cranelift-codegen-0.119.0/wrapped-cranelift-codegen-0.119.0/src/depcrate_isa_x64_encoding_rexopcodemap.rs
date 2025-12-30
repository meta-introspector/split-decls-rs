// Generated macro for OpcodeMap (enum)
macro_rules! Depcrate_isa_x64_encoding_rexOpcodeMap {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"OpcodeMap"}
// Dependencies: {}
# [doc = " Allows using the same opcode byte in different \"opcode maps\" to allow for more instruction"] # [doc = " encodings. See appendix A in the Intel Software Developer's Manual, volume 2A, for more details."] # [allow (missing_docs)] # [derive (PartialEq)] pub enum OpcodeMap { None , _0F , _0F38 , _0F3A , }
};
}
