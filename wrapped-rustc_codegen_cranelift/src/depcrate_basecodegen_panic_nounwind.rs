// Generated macro for codegen_panic_nounwind (function)
macro_rules! Depcrate_basecodegen_panic_nounwind {
() => {
// Module: crate::base
// Provides: {"codegen_panic_nounwind"}
// Dependencies: {}
pub (crate) fn codegen_panic_nounwind < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , msg_str : & str , span : Span ,) { let msg_ptr = fx . anonymous_str (msg_str) ; let msg_len = fx . bcx . ins () . iconst (fx . pointer_type , i64 :: try_from (msg_str . len ()) . unwrap ()) ; let args = [msg_ptr , msg_len] ; codegen_panic_inner (fx , rustc_hir :: LangItem :: PanicNounwind , & args , UnwindAction :: Terminate (UnwindTerminateReason :: Abi) , span ,) ; }
};
}
