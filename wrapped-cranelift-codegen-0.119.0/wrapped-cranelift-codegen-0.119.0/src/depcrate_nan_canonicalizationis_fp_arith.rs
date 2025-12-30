// Generated macro for is_fp_arith (function)
macro_rules! Depcrate_nan_canonicalizationis_fp_arith {
() => {
// Module: crate::nan_canonicalization
// Provides: {"is_fp_arith"}
// Dependencies: {}
# [doc = " Returns true/false based on whether the instruction is a floating-point"] # [doc = " arithmetic operation. This ignores operations like `fneg`, `fabs`, or"] # [doc = " `fcopysign` that only operate on the sign bit of a floating point value."] fn is_fp_arith (pos : & mut FuncCursor , inst : Inst) -> bool { match pos . func . dfg . insts [inst] { InstructionData :: Unary { opcode , .. } => { opcode == Opcode :: Ceil || opcode == Opcode :: Floor || opcode == Opcode :: Nearest || opcode == Opcode :: Sqrt || opcode == Opcode :: Trunc || opcode == Opcode :: Fdemote || opcode == Opcode :: Fpromote || opcode == Opcode :: FvpromoteLow || opcode == Opcode :: Fvdemote } InstructionData :: Binary { opcode , .. } => { opcode == Opcode :: Fadd || opcode == Opcode :: Fdiv || opcode == Opcode :: Fmax || opcode == Opcode :: Fmin || opcode == Opcode :: Fmul || opcode == Opcode :: Fsub } InstructionData :: Ternary { opcode , .. } => opcode == Opcode :: Fma , _ => false , } }
};
}
