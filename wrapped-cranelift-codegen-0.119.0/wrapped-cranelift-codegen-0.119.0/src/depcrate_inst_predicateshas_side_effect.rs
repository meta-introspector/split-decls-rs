// Generated macro for has_side_effect (function)
macro_rules! Depcrate_inst_predicateshas_side_effect {
() => {
// Module: crate::inst_predicates
// Provides: {"has_side_effect"}
// Dependencies: {}
# [doc = " Does the given instruction have any side-effect that would preclude it from being removed when"] # [doc = " its value is unused?"] # [inline (always)] fn has_side_effect (func : & Function , inst : Inst) -> bool { let data = & func . dfg . insts [inst] ; let opcode = data . opcode () ; trivially_has_side_effects (opcode) || is_load_with_defined_trapping (opcode , data) }
};
}
