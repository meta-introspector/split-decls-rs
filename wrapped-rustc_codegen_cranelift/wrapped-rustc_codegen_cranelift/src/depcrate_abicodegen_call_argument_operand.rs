// Generated macro for codegen_call_argument_operand (function)
macro_rules! Depcrate_abicodegen_call_argument_operand {
() => {
// Module: crate::abi
// Provides: {"codegen_call_argument_operand"}
// Dependencies: {}
fn codegen_call_argument_operand < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , operand : & Operand < 'tcx > ,) -> CallArgument < 'tcx > { CallArgument { value : codegen_operand (fx , operand) , is_owned : matches ! (operand , Operand :: Move (_)) , } }
};
}
