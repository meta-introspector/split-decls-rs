// Generated macro for input_to_imm (function)
macro_rules! Depcrate_isa_x64_lowerinput_to_imm {
() => {
// Module: crate::isa::x64::lower
// Provides: {"input_to_imm"}
// Dependencies: {}
fn input_to_imm (ctx : & mut Lower < Inst > , spec : InsnInput) -> Option < u64 > { ctx . get_input_as_source_or_const (spec . insn , spec . input) . constant }
};
}
