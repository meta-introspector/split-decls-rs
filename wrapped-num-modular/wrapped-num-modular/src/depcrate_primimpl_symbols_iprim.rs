// Generated macro for impl_symbols_iprim (macro)
macro_rules! Depcrate_primimpl_symbols_iprim {
() => {
// Module: crate::prim
// Provides: {"impl_symbols_iprim"}
// Dependencies: {}
macro_rules ! impl_symbols_iprim { ($ ($ T : ty , $ U : ty ;) *) => ($ (impl ModularSymbols <&$ T > for $ T { # [inline] fn checked_legendre (& self , n : &$ T) -> Option < i8 > { if n < & 1 { return None ; } let a = self . rem_euclid (* n) as $ U ; a . checked_legendre (& (* n as $ U)) } # [inline] fn checked_jacobi (& self , n : &$ T) -> Option < i8 > { if n < & 1 { return None ; } let a = self . rem_euclid (* n) as $ U ; a . checked_jacobi (& (* n as $ U)) } # [inline] fn kronecker (& self , n : &$ T) -> i8 { match n { - 1 => { if self < & 0 { - 1 } else { 1 } } 0 => { if self == & 1 { 1 } else { 0 } } 1 => 1 , 2 => { if self % 2 == 0 { 0 } else if self . rem_euclid (8) == 1 || self . rem_euclid (8) == 7 { 1 } else { - 1 } } , i if i < &- 1 => { self . kronecker (&- 1) * self . kronecker (&- i) } , _ => { let f = n . trailing_zeros () ; self . kronecker (& 2) . pow (f) * self . jacobi (& (n >> f)) } } } }) *) ; }
};
}
