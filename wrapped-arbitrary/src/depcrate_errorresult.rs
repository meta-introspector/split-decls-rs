// Generated macro for Result (type)
macro_rules! Depcrate_errorResult {
() => {
// Module: crate::error
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A `Result` with the error type fixed as `arbitrary::Error`."] # [doc = ""] # [doc = " Either an `Ok(T)` or `Err(arbitrary::Error)`."] pub type Result < T , E = Error > = std :: result :: Result < T , E > ;
};
}
