// Generated macro for impl_unary_uprim (macro)
macro_rules! Depcrate_primimpl_unary_uprim {
() => {
// Module: crate::prim
// Provides: {"impl_unary_uprim"}
// Dependencies: {}
macro_rules ! impl_unary_uprim { ($ ($ T : ty) *) => ($ (impl ModularUnaryOps <&$ T > for $ T { type Output = $ T ; # [inline] fn negm (self , m : &$ T) -> $ T { let x = self % m ; if x == 0 { 0 } else { m - x } } fn invm (self , m : &$ T) -> Option <$ T > { let x = if & self >= m { self % m } else { self . clone () } ; let (mut last_r , mut r) = (m . clone () , x) ; let (mut last_t , mut t) = (0 , 1) ; while r > 0 { let (quo , rem) = (last_r / r , last_r % r) ; last_r = r ; r = rem ; let new_t = last_t . subm (quo . mulm (t , m) , m) ; last_t = t ; t = new_t ; } if last_r > 1 { None } else { Some (last_t) } } # [inline (always)] fn dblm (self , m : &$ T) -> $ T { self . addm (self , m) } # [inline (always)] fn sqm (self , m : &$ T) -> $ T { self . mulm (self , m) } }) *) ; }
};
}
