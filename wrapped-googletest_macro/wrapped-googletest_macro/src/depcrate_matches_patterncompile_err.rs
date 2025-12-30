// Generated macro for compile_err (function)
macro_rules! Depcrate_matches_patterncompile_err {
() => {
// Module: crate::matches_pattern
// Provides: {"compile_err"}
// Dependencies: {}
fn compile_err < T > (span : Span , message : & str) -> syn :: Result < T > { Err (syn :: Error :: new (span , message)) }
};
}
