// Generated macro for impl_19 (impl)
macro_rules! Depcrate_exprimpl_19 {
() => {
// Module: crate::expr
// Provides: {"impl_19"}
// Dependencies: {}
impl EvalResult { result_opt ! (fn as_int : Int -> Wrapping < i64 >) ; result_opt ! (fn as_float : Float -> f64) ; result_opt ! (fn as_char : Char -> CChar) ; result_opt ! (fn as_str : Str -> Vec < u8 >) ; # [allow (clippy :: wrong_self_convention)] fn as_numeric (self) -> Option < EvalResult > { match self { EvalResult :: Int (_) | EvalResult :: Float (_) => Some (self) , _ => None , } } }
};
}
