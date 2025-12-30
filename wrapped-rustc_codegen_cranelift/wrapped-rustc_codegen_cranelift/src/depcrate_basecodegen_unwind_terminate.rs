// Generated macro for codegen_unwind_terminate (function)
macro_rules! Depcrate_basecodegen_unwind_terminate {
() => {
// Module: crate::base
// Provides: {"codegen_unwind_terminate"}
// Dependencies: {}
pub (crate) fn codegen_unwind_terminate < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , span : Span , reason : UnwindTerminateReason ,) { codegen_panic_inner (fx , reason . lang_item () , & [] , UnwindAction :: Unreachable , span) ; }
};
}
