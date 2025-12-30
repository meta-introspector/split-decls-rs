// Generated macro for numeric (function)
macro_rules! Depcrate_exprnumeric {
() => {
// Module: crate::expr
// Provides: {"numeric"}
// Dependencies: {}
fn numeric < I : Clone , E : nom :: error :: ParseError < I > , F > (f : F ,) -> impl FnMut (I) -> nom :: IResult < I , EvalResult , E > where F : FnMut (I) -> nom :: IResult < I , EvalResult , E > , { nom :: combinator :: map_opt (f , EvalResult :: as_numeric) }
};
}
