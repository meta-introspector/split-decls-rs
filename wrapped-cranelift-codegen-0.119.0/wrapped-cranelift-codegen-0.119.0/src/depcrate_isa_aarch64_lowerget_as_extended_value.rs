// Generated macro for get_as_extended_value (function)
macro_rules! Depcrate_isa_aarch64_lowerget_as_extended_value {
() => {
// Module: crate::isa::aarch64::lower
// Provides: {"get_as_extended_value"}
// Dependencies: {}
fn get_as_extended_value (ctx : & mut Lower < Inst > , val : Value) -> Option < (Value , ExtendOp) > { let inputs = ctx . get_value_as_source_or_const (val) ; let (insn , n) = inputs . inst . as_inst () ? ; if n != 0 { return None ; } let op = ctx . data (insn) . opcode () ; let out_ty = ctx . output_ty (insn , 0) ; let out_bits = ty_bits (out_ty) ; if op == Opcode :: Uextend || op == Opcode :: Sextend { let sign_extend = op == Opcode :: Sextend ; let inner_ty = ctx . input_ty (insn , 0) ; let inner_bits = ty_bits (inner_ty) ; assert ! (inner_bits < out_bits) ; let extendop = match (sign_extend , inner_bits) { (true , 8) => ExtendOp :: SXTB , (false , 8) => ExtendOp :: UXTB , (true , 16) => ExtendOp :: SXTH , (false , 16) => ExtendOp :: UXTH , (true , 32) => ExtendOp :: SXTW , (false , 32) => ExtendOp :: UXTW , _ => unreachable ! () , } ; return Some ((ctx . input_as_value (insn , 0) , extendop)) ; } None }
};
}
