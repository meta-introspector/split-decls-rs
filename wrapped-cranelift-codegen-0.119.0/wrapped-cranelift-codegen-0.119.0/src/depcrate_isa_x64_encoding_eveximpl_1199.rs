// Generated macro for impl_1199 (impl)
macro_rules! Depcrate_isa_x64_encoding_eveximpl_1199 {
() => {
// Module: crate::isa::x64::encoding::evex
// Provides: {"impl_1199"}
// Dependencies: {}
# [doc = " Because some of the bit flags in the EVEX prefix are reversed and users of `EvexInstruction` may"] # [doc = " choose to skip setting fields, here we set some sane defaults. Note that:"] # [doc = " - the first byte is always `0x62` but you will notice it at the end of the default `bits` value"] # [doc = "   implemented--remember the little-endian order"] # [doc = " - some bits are always set to certain values: bits 10-11 to 0, bit 18 to 1"] # [doc = " - the other bits set correspond to reversed bits: R, X, B, R' (byte 1), vvvv (byte 2), V' (byte"] # [doc = "   3)."] # [doc = ""] # [doc = " See the `default_emission` test for what these defaults are equivalent to (e.g. using RAX,"] # [doc = " unsetting the W bit, etc.)"] impl Default for EvexInstruction { fn default () -> Self { Self { bits : 0x08_7C_F0_62 , opcode : 0 , reg : Register :: default () , rm : RegisterOrAmode :: Register (Register :: default ()) , tuple_type : None , imm : None , } } }
};
}
