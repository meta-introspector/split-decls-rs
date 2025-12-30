// Generated macro for is_pure_for_egraph (function)
macro_rules! Depcrate_inst_predicatesis_pure_for_egraph {
() => {
// Module: crate::inst_predicates
// Provides: {"is_pure_for_egraph"}
// Dependencies: {}
# [doc = " Does the given instruction behave as a \"pure\" node with respect to"] # [doc = " aegraph semantics?"] # [doc = ""] # [doc = " - Trivially pure nodes (bitwise arithmetic, etc)"] # [doc = " - Loads with the `readonly`, `notrap`, and `can_move` flags set"] pub fn is_pure_for_egraph (func : & Function , inst : Inst) -> bool { let is_pure_load = match func . dfg . insts [inst] { InstructionData :: Load { opcode : Opcode :: Load , flags , .. } => flags . readonly () && flags . notrap () && flags . can_move () , _ => false , } ; let has_one_result = func . dfg . inst_results (inst) . len () == 1 ; let op = func . dfg . insts [inst] . opcode () ; has_one_result && (is_pure_load || (! op . can_load () && ! trivially_has_side_effects (op))) }
};
}
