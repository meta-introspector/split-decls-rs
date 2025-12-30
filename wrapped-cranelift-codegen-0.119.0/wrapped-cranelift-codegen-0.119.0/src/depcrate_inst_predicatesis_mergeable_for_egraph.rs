// Generated macro for is_mergeable_for_egraph (function)
macro_rules! Depcrate_inst_predicatesis_mergeable_for_egraph {
() => {
// Module: crate::inst_predicates
// Provides: {"is_mergeable_for_egraph"}
// Dependencies: {}
# [doc = " Can the given instruction be merged into another copy of itself?"] # [doc = " These instructions may have side-effects, but as long as we retain"] # [doc = " the first instance of the instruction, the second and further"] # [doc = " instances are redundant if they would produce the same trap or"] # [doc = " result."] pub fn is_mergeable_for_egraph (func : & Function , inst : Inst) -> bool { let op = func . dfg . insts [inst] . opcode () ; func . dfg . inst_results (inst) . len () <= 1 && ! op . can_load () && ! op . can_store () && (! has_side_effect (func , inst) || op . side_effects_idempotent ()) }
};
}
