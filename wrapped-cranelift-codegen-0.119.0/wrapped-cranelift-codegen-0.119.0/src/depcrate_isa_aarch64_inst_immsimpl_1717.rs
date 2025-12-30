// Generated macro for impl_1717 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1717 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1717"}
// Dependencies: {}
impl PrettyPrint for ASIMDMovModImm { fn pretty_print (& self , _ : u8) -> String { if self . is_64bit { debug_assert_eq ! (self . shift , 0) ; let enc_imm = self . imm as i8 ; let mut imm = 0u64 ; for i in 0 .. 8 { let b = (enc_imm >> i) & 1 ; imm |= (- b as u8 as u64) << (i * 8) ; } format ! ("#{imm}") } else if self . shift == 0 { format ! ("#{}" , self . imm) } else { let shift_type = if self . shift_ones { "MSL" } else { "LSL" } ; format ! ("#{}, {} #{}" , self . imm , shift_type , self . shift) } } }
};
}
