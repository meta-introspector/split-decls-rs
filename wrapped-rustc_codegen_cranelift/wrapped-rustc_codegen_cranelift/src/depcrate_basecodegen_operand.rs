// Generated macro for codegen_operand (function)
macro_rules! Depcrate_basecodegen_operand {
() => {
// Module: crate::base
// Provides: {"codegen_operand"}
// Dependencies: {}
pub (crate) fn codegen_operand < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , operand : & Operand < 'tcx > ,) -> CValue < 'tcx > { match operand { Operand :: Move (place) | Operand :: Copy (place) => { let cplace = codegen_place (fx , * place) ; cplace . to_cvalue (fx) } Operand :: Constant (const_) => crate :: constant :: codegen_constant_operand (fx , const_) , } }
};
}
