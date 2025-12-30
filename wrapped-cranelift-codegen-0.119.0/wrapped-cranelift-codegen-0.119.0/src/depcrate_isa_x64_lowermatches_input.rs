// Generated macro for matches_input (function)
macro_rules! Depcrate_isa_x64_lowermatches_input {
() => {
// Module: crate::isa::x64::lower
// Provides: {"matches_input"}
// Dependencies: {}
# [doc = " Returns whether the given specified `input` is a result produced by an instruction with Opcode"] # [doc = " `op`."] fn matches_input (ctx : & mut Lower < Inst > , input : InsnInput , op : Opcode) -> Option < IRInst > { let inputs = ctx . get_input_as_source_or_const (input . insn , input . input) ; inputs . inst . as_inst () . and_then (| (src_inst , _) | { let data = ctx . data (src_inst) ; if data . opcode () == op { return Some (src_inst) ; } None }) }
};
}
