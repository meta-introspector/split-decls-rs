// Generated macro for CompileResult (type)
macro_rules! Depcrate_resultCompileResult {
() => {
// Module: crate::result
// Provides: {"CompileResult"}
// Dependencies: {}
# [doc = " A convenient alias for a `Result` that uses `CompileError` as the error type."] pub type CompileResult < 'a , T > = Result < T , CompileError < 'a > > ;
};
}
