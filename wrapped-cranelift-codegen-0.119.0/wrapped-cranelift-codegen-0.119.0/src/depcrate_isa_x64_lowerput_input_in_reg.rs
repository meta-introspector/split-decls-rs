// Generated macro for put_input_in_reg (function)
macro_rules! Depcrate_isa_x64_lowerput_input_in_reg {
() => {
// Module: crate::isa::x64::lower
// Provides: {"put_input_in_reg"}
// Dependencies: {}
# [doc = " Put the given input into a register, and mark it as used (side-effect)."] fn put_input_in_reg (ctx : & mut Lower < Inst > , spec : InsnInput) -> Reg { put_input_in_regs (ctx , spec) . only_reg () . expect ("Multi-register value not expected") }
};
}
