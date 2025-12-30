// Generated macro for enc_bfm (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_bfm {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_bfm"}
// Dependencies: {}
fn enc_bfm (opc : u8 , size : OperandSize , rd : Writable < Reg > , rn : Reg , immr : u8 , imms : u8) -> u32 { match size { OperandSize :: Size64 => { debug_assert ! (immr <= 63) ; debug_assert ! (imms <= 63) ; } OperandSize :: Size32 => { debug_assert ! (immr <= 31) ; debug_assert ! (imms <= 31) ; } } debug_assert_eq ! (opc & 0b11 , opc) ; let n_bit = size . sf_bit () ; 0b0_00_100110_0_000000_000000_00000_00000 | size . sf_bit () << 31 | u32 :: from (opc) << 29 | n_bit << 22 | u32 :: from (immr) << 16 | u32 :: from (imms) << 10 | machreg_to_gpr (rn) << 5 | machreg_to_gpr (rd . to_reg ()) }
};
}
