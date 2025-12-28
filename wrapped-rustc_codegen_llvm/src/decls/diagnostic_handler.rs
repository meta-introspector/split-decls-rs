macro_rules! deps {
    () => {
        Diagnostic!();
        FromLlvmDiag!();
        LlvmCodegenBackend!();
        Linker!();
        FromLlvmOptimizationDiag!();
    };
}

macro_rules! diagnostic_handler {
    () => {
        deps!();
        unsafe extern "C" fn diagnostic_handler (info : & DiagnosticInfo , user : * mut c_void) { if user . is_null () { return ; } let (cgcx , dcx) = unsafe { * (user as * const (& CodegenContext < LlvmCodegenBackend > , DiagCtxtHandle < '_ >)) } ; match unsafe { llvm :: diagnostic :: Diagnostic :: unpack (info) } { llvm :: diagnostic :: InlineAsm (inline) => { report_inline_asm (cgcx , inline . message , inline . level , inline . cookie , inline . source) ; } llvm :: diagnostic :: Optimization (opt) => { dcx . emit_note (FromLlvmOptimizationDiag { filename : & opt . filename , line : opt . line , column : opt . column , pass_name : & opt . pass_name , kind : match opt . kind { OptimizationRemark => "success" , OptimizationMissed | OptimizationFailure => "missed" , OptimizationAnalysis | OptimizationAnalysisFPCommute | OptimizationAnalysisAliasing => "analysis" , OptimizationRemarkOther => "other" , } , message : & opt . message , }) ; } llvm :: diagnostic :: PGO (diagnostic_ref) | llvm :: diagnostic :: Linker (diagnostic_ref) => { let message = llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteDiagnosticInfoToString (diagnostic_ref , s) }) . expect ("non-UTF8 diagnostic") ; dcx . emit_warn (FromLlvmDiag { message }) ; } llvm :: diagnostic :: Unsupported (diagnostic_ref) => { let message = llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteDiagnosticInfoToString (diagnostic_ref , s) }) . expect ("non-UTF8 diagnostic") ; dcx . emit_err (FromLlvmDiag { message }) ; } llvm :: diagnostic :: UnknownDiagnostic (..) => { } } }
    };
}

diagnostic_handler!();