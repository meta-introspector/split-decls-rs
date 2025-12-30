// Generated macro for impl_933 (impl)
macro_rules! Depcrate_llvm_diagnosticimpl_933 {
() => {
// Module: crate::llvm::diagnostic
// Provides: {"impl_933"}
// Dependencies: {}
impl InlineAsmDiagnostic { unsafe fn unpackInlineAsm (di : & DiagnosticInfo) -> Self { let mut cookie = 0 ; let mut message = None ; let mut level = super :: DiagnosticLevel :: Error ; unsafe { super :: LLVMRustUnpackInlineAsmDiagnostic (di , & mut level , & mut cookie , & mut message) ; } InlineAsmDiagnostic { level , cookie , message : super :: twine_to_string (message . unwrap ()) , source : None , } } unsafe fn unpackSrcMgr (di : & DiagnosticInfo) -> Self { let mut cookie = 0 ; let smdiag = unsafe { SrcMgrDiagnostic :: unpack (super :: LLVMRustGetSMDiagnostic (di , & mut cookie)) } ; InlineAsmDiagnostic { level : smdiag . level , cookie , message : smdiag . message , source : smdiag . source , } } }
};
}
