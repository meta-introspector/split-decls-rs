// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl PatternResult { fn score (& self) -> f64 { match * self { PatternResult :: Linear (score) | PatternResult :: NonLinear (score) => score , PatternResult :: TooLong => 0.0 , } } }
};
}
