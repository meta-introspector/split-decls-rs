// Generated macro for trivially_has_side_effects (function)
macro_rules! Depcrate_inst_predicatestrivially_has_side_effects {
() => {
// Module: crate::inst_predicates
// Provides: {"trivially_has_side_effects"}
// Dependencies: {}
# [doc = " Test whether the given opcode is unsafe to even consider as side-effect-free."] # [inline (always)] fn trivially_has_side_effects (opcode : Opcode) -> bool { opcode . is_call () || opcode . is_branch () || opcode . is_terminator () || opcode . is_return () || opcode . can_trap () || opcode . other_side_effects () || opcode . can_store () }
};
}
