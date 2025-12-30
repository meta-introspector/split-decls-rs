// Generated macro for CompilerBuiltinsCannotCall (struct)
macro_rules! Depcrate_errorsCompilerBuiltinsCannotCall {
() => {
// Module: crate::errors
// Provides: {"CompilerBuiltinsCannotCall"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_ssa_compiler_builtins_cannot_call)] pub struct CompilerBuiltinsCannotCall { pub caller : String , pub callee : String , # [primary_span] pub span : Span , }
};
}
