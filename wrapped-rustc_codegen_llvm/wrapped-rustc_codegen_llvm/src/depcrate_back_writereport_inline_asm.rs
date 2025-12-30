// Generated macro for report_inline_asm (function)
macro_rules! Depcrate_back_writereport_inline_asm {
() => {
// Module: crate::back::write
// Provides: {"report_inline_asm"}
// Dependencies: {}
fn report_inline_asm (cgcx : & CodegenContext < LlvmCodegenBackend > , msg : String , level : llvm :: DiagnosticLevel , cookie : u64 , source : Option < (String , Vec < InnerSpan >) > ,) { let span = if cookie == 0 || matches ! (cgcx . lto , Lto :: Fat | Lto :: Thin) { SpanData :: default () } else { SpanData { lo : BytePos :: from_u32 (cookie as u32) , hi : BytePos :: from_u32 ((cookie >> 32) as u32) , ctxt : SyntaxContext :: root () , parent : None , } } ; let level = match level { llvm :: DiagnosticLevel :: Error => Level :: Error , llvm :: DiagnosticLevel :: Warning => Level :: Warning , llvm :: DiagnosticLevel :: Note | llvm :: DiagnosticLevel :: Remark => Level :: Note , } ; let msg = msg . strip_prefix ("error: ") . unwrap_or (& msg) . to_string () ; cgcx . diag_emitter . inline_asm_error (span , msg , level , source) ; }
};
}
