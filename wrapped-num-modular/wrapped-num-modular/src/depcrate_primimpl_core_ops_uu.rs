// Generated macro for impl_core_ops_uu (macro)
macro_rules! Depcrate_primimpl_core_ops_uu {
() => {
// Module: crate::prim
// Provides: {"impl_core_ops_uu"}
// Dependencies: {}
macro_rules ! impl_core_ops_uu { ($ ($ T : ty => $ Tdouble : ty ;) *) => ($ (impl ModularCoreOps <$ T , &$ T > for $ T { type Output = $ T ; # [inline (always)] fn addm (self , rhs : $ T , m : &$ T) -> $ T { (((self as $ Tdouble) + (rhs as $ Tdouble)) % (* m as $ Tdouble)) as $ T } # [inline] fn subm (self , rhs : $ T , m : &$ T) -> $ T { if self >= rhs { (self - rhs) % m } else { ((rhs - self) % m) . negm (m) } } # [inline (always)] fn mulm (self , rhs : $ T , m : &$ T) -> $ T { (((self as $ Tdouble) * (rhs as $ Tdouble)) % (* m as $ Tdouble)) as $ T } }) *) ; }
};
}
