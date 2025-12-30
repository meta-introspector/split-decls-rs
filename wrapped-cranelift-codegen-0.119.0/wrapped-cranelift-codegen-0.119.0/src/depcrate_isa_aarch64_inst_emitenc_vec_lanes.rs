// Generated macro for enc_vec_lanes (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_vec_lanes {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_vec_lanes"}
// Dependencies: {}
fn enc_vec_lanes (q : u32 , u : u32 , size : u32 , opcode : u32 , rd : Writable < Reg > , rn : Reg) -> u32 { debug_assert_eq ! (q & 0b1 , q) ; debug_assert_eq ! (u & 0b1 , u) ; debug_assert_eq ! (size & 0b11 , size) ; debug_assert_eq ! (opcode & 0b11111 , opcode) ; 0b0_0_0_01110_00_11000_0_0000_10_00000_00000 | q << 30 | u << 29 | size << 22 | opcode << 12 | machreg_to_vec (rn) << 5 | machreg_to_vec (rd . to_reg ()) }
};
}
