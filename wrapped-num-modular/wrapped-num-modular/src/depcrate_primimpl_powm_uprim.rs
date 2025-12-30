// Generated macro for impl_powm_uprim (macro)
macro_rules! Depcrate_primimpl_powm_uprim {
() => {
// Module: crate::prim
// Provides: {"impl_powm_uprim"}
// Dependencies: {}
macro_rules ! impl_powm_uprim { ($ ($ T : ty) *) => ($ (impl ModularPow <$ T , &$ T > for $ T { type Output = $ T ; # [inline (always)] fn powm (self , exp : $ T , m : &$ T) -> $ T { Vanilla ::<$ T >:: new (& m) . pow (self % m , & exp) } }) *) ; }
};
}
