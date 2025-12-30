// Generated macro for impl_1718 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1718 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1718"}
// Dependencies: {}
impl PrettyPrint for ASIMDFPModImm { fn pretty_print (& self , _ : u8) -> String { match self . size { ScalarSize :: Size16 => { let value : u32 = Self :: value16 (self . imm) . into () ; let sign = (value & 0x8000) << 16 ; let exponent = ((value & 0x7c00) + ((127 - 15) << 10)) << 13 ; let significand = (value & 0x3ff) << 13 ; format ! ("#{}" , f32 :: from_bits (sign | exponent | significand)) } ScalarSize :: Size32 => format ! ("#{}" , f32 :: from_bits (Self :: value32 (self . imm))) , ScalarSize :: Size64 => format ! ("#{}" , f64 :: from_bits (Self :: value64 (self . imm))) , _ => unreachable ! () , } } }
};
}
