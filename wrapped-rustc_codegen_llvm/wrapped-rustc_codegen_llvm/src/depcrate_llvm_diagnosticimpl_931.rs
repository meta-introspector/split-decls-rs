// Generated macro for impl_931 (impl)
macro_rules! Depcrate_llvm_diagnosticimpl_931 {
() => {
// Module: crate::llvm::diagnostic
// Provides: {"impl_931"}
// Dependencies: {}
impl SrcMgrDiagnostic { pub (crate) unsafe fn unpack (diag : & SMDiagnostic) -> SrcMgrDiagnostic { let mut have_source = false ; let mut buffer = String :: new () ; let mut level = super :: DiagnosticLevel :: Error ; let mut loc = 0 ; let mut ranges = [0 ; 8] ; let mut num_ranges = ranges . len () / 2 ; let message = super :: build_string (| message | { buffer = super :: build_string (| buffer | unsafe { have_source = super :: LLVMRustUnpackSMDiagnostic (diag , message , buffer , & mut level , & mut loc , ranges . as_mut_ptr () , & mut num_ranges ,) ; }) . expect ("non-UTF8 inline asm") ; }) . expect ("non-UTF8 SMDiagnostic") ; SrcMgrDiagnostic { message , level , source : have_source . then (| | { let mut spans = vec ! [InnerSpan :: new (loc as usize , loc as usize)] ; for i in 0 .. num_ranges { spans . push (InnerSpan :: new (ranges [i * 2] as usize , ranges [i * 2 + 1] as usize)) ; } (buffer , spans) }) , } } }
};
}
