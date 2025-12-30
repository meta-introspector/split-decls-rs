// Generated macro for enc_cas (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_cas {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_cas"}
// Dependencies: {}
fn enc_cas (size : u32 , rs : Writable < Reg > , rt : Reg , rn : Reg) -> u32 { debug_assert_eq ! (size & 0b11 , size) ; 0b00_0010001_1_1_00000_1_11111_00000_00000 | size << 30 | machreg_to_gpr (rs . to_reg ()) << 16 | machreg_to_gpr (rn) << 5 | machreg_to_gpr (rt) }
};
}
