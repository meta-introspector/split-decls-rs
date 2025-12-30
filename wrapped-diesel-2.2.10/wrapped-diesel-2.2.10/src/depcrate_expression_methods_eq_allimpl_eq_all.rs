// Generated macro for impl_eq_all (macro)
macro_rules! Depcrate_expression_methods_eq_allimpl_eq_all {
() => {
// Module: crate::expression_methods::eq_all
// Provides: {"impl_eq_all"}
// Dependencies: {}
macro_rules ! impl_eq_all { (($ Left1 : ident , $ ($ Left : ident ,) +) ($ Right1 : ident , $ ($ Right : ident ,) +)) => { # [allow (non_snake_case)] impl <$ Left1 , $ ($ Left ,) + $ Right1 , $ ($ Right ,) +> EqAll < ($ Right1 , $ ($ Right ,) +) > for ($ Left1 , $ ($ Left ,) +) where $ Left1 : EqAll <$ Right1 >, ($ ($ Left ,) +) : EqAll < ($ ($ Right ,) +) >, { type Output = Grouped < And < <$ Left1 as EqAll <$ Right1 >>:: Output , < ($ ($ Left ,) +) as EqAll < ($ ($ Right ,) +) >>:: Output , >>; fn eq_all (self , rhs : ($ Right1 , $ ($ Right ,) +)) -> Self :: Output { let ($ Left1 , $ ($ Left ,) +) = self ; let ($ Right1 , $ ($ Right ,) +) = rhs ; $ Left1 . eq_all ($ Right1) . and (($ ($ Left ,) +) . eq_all (($ ($ Right ,) +))) } } } ; (($ Left : ident ,) ($ Right : ident ,)) => { impl <$ Left , $ Right > EqAll < ($ Right ,) > for ($ Left ,) where $ Left : EqAll <$ Right >, { type Output = <$ Left as EqAll <$ Right >>:: Output ; fn eq_all (self , rhs : ($ Right ,)) -> Self :: Output { self . 0 . eq_all (rhs . 0) } } } ; }
};
}
