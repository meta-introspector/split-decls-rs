// Generated macro for approx_eq (macro)
macro_rules! Depcrate_macrosapprox_eq {
() => {
// Module: crate::macros
// Provides: {"approx_eq"}
// Dependencies: {}
# [macro_export] macro_rules ! approx_eq { ($ typ : ty , $ lhs : expr , $ rhs : expr) => { { let m = <$ typ as $ crate :: ApproxEq >:: Margin :: default () ; <$ typ as $ crate :: ApproxEq >:: approx_eq ($ lhs , $ rhs , m) } } ; ($ typ : ty , $ lhs : expr , $ rhs : expr $ (, $ set : ident = $ val : expr) *) => { { use $ crate :: FloatMargin ; let m = <$ typ as $ crate :: ApproxEq >:: Margin :: zero () $ (.$ set ($ val)) *; <$ typ as $ crate :: ApproxEq >:: approx_eq ($ lhs , $ rhs , m) } } ; ($ typ : ty , $ lhs : expr , $ rhs : expr , $ marg : expr) => { { <$ typ as $ crate :: ApproxEq >:: approx_eq ($ lhs , $ rhs , $ marg) } } ; }
};
}
