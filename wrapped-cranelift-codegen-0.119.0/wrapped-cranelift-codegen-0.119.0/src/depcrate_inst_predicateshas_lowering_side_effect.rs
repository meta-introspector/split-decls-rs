// Generated macro for has_lowering_side_effect (function)
macro_rules! Depcrate_inst_predicateshas_lowering_side_effect {
() => {
// Module: crate::inst_predicates
// Provides: {"has_lowering_side_effect"}
// Dependencies: {}
# [doc = " Does the given instruction have any side-effect as per [has_side_effect], or else is a load,"] # [doc = " but not the get_pinned_reg opcode?"] pub fn has_lowering_side_effect (func : & Function , inst : Inst) -> bool { let op = func . dfg . insts [inst] . opcode () ; op != Opcode :: GetPinnedReg && (has_side_effect (func , inst) || op . can_load ()) }
};
}
