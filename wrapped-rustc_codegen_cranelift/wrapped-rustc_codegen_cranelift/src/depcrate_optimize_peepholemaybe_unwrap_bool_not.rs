// Generated macro for maybe_unwrap_bool_not (function)
macro_rules! Depcrate_optimize_peepholemaybe_unwrap_bool_not {
() => {
// Module: crate::optimize::peephole
// Provides: {"maybe_unwrap_bool_not"}
// Dependencies: {}
# [doc = " If the given value was produced by the lowering of `Rvalue::Not` return the input and true,"] # [doc = " otherwise return the given value and false."] pub (crate) fn maybe_unwrap_bool_not (bcx : & mut FunctionBuilder < '_ > , arg : Value) -> (Value , bool) { if let ValueDef :: Result (arg_inst , 0) = bcx . func . dfg . value_def (arg) { match bcx . func . dfg . insts [arg_inst] { InstructionData :: IntCompareImm { opcode : Opcode :: IcmpImm , cond : IntCC :: Equal , arg , imm , } if imm . bits () == 0 => (arg , true) , _ => (arg , false) , } } else { (arg , false) } }
};
}
