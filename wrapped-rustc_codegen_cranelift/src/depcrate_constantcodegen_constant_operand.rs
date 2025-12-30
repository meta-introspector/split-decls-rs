// Generated macro for codegen_constant_operand (function)
macro_rules! Depcrate_constantcodegen_constant_operand {
() => {
// Module: crate::constant
// Provides: {"codegen_constant_operand"}
// Dependencies: {}
pub (crate) fn codegen_constant_operand < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , constant : & ConstOperand < 'tcx > ,) -> CValue < 'tcx > { let (const_val , ty) = eval_mir_constant (fx , constant) ; codegen_const_value (fx , const_val , ty) }
};
}
