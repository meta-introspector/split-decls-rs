// Generated macro for factorial (function)
macro_rules! Depcratefactorial {
() => {
// Module: crate
// Provides: {"factorial"}
// Dependencies: {}
# [trace_var (p , n)] fn factorial (mut n : u64) -> u64 { let mut p = 1 ; while n > 1 { p *= n ; n -= 1 ; } p }
};
}
