// Generated macro for impl_CFComparison (macro)
macro_rules! Depcrateimpl_CFComparison {
() => {
// Module: crate
// Provides: {"impl_CFComparison"}
// Dependencies: {}
# [macro_export] macro_rules ! impl_CFComparison { ($ ty : ident , $ compare : ident) => { impl_CFComparison ! ($ ty <>, $ compare) ; } ; ($ ty : ident <$ ($ p : ident $ (: $ bound : path) *) ,*>, $ compare : ident) => { impl <$ ($ p $ (: $ bound) *) ,*> PartialOrd for $ ty <$ ($ p) ,*> { # [inline] fn partial_cmp (& self , other : &$ ty <$ ($ p) ,*>) -> Option <:: std :: cmp :: Ordering > { unsafe { Some ($ compare (self . as_concrete_TypeRef () , other . as_concrete_TypeRef () , :: std :: ptr :: null_mut () ,) . into () ,) } } } impl <$ ($ p $ (: $ bound) *) ,*> Ord for $ ty <$ ($ p) ,*> { # [inline] fn cmp (& self , other : &$ ty <$ ($ p) ,*>) -> :: std :: cmp :: Ordering { self . partial_cmp (other) . unwrap () } } } ; }
};
}
