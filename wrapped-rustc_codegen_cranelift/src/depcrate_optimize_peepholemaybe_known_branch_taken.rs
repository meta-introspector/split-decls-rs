// Generated macro for maybe_known_branch_taken (function)
macro_rules! Depcrate_optimize_peepholemaybe_known_branch_taken {
() => {
// Module: crate::optimize::peephole
// Provides: {"maybe_known_branch_taken"}
// Dependencies: {}
# [doc = " Returns whether the branch is statically known to be taken or `None` if it isn't statically known."] pub (crate) fn maybe_known_branch_taken (bcx : & FunctionBuilder < '_ > , arg : Value , test_zero : bool ,) -> Option < bool > { let arg_inst = if let ValueDef :: Result (arg_inst , 0) = bcx . func . dfg . value_def (arg) { arg_inst } else { return None ; } ; match bcx . func . dfg . insts [arg_inst] { InstructionData :: UnaryImm { opcode : Opcode :: Iconst , imm } => { if test_zero { Some (imm . bits () == 0) } else { Some (imm . bits () != 0) } } _ => None , } }
};
}
