// Generated macro for CompileError (struct)
macro_rules! Depcrate_resultCompileError {
() => {
// Module: crate::result
// Provides: {"CompileError"}
// Dependencies: {}
# [doc = " Compilation error, with the accompanying function to help printing it."] pub struct CompileError < 'a > { # [doc = " Underlying `CodegenError` that triggered the error."] pub inner : CodegenError , # [doc = " Function we tried to compile, for display purposes."] pub func : & 'a Function , }
};
}
