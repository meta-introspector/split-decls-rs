// Generated macro for impl_symbols_uprim (macro)
macro_rules! Depcrate_primimpl_symbols_uprim {
() => {
// Module: crate::prim
// Provides: {"impl_symbols_uprim"}
// Dependencies: {}
macro_rules ! impl_symbols_uprim { ($ ($ T : ty) *) => ($ (impl ModularSymbols <&$ T > for $ T { # [inline] fn checked_legendre (& self , n : &$ T) -> Option < i8 > { match self . powm ((n - 1) / 2 , & n) { 0 => Some (0) , 1 => Some (1) , x if x == n - 1 => Some (- 1) , _ => None , } } fn checked_jacobi (& self , n : &$ T) -> Option < i8 > { if n % 2 == 0 { return None ; } if self == & 0 { return Some (if n == & 1 { 1 } else { 0 }) ; } if self == & 1 { return Some (1) ; } let mut a = self % n ; let mut n = * n ; let mut t = 1 ; while a > 0 { while a % 2 == 0 { a /= 2 ; if n % 8 == 3 || n % 8 == 5 { t *= - 1 ; } } core :: mem :: swap (& mut a , & mut n) ; if a % 4 == 3 && n % 4 == 3 { t *= - 1 ; } a %= n ; } Some (if n == 1 { t } else { 0 }) } fn kronecker (& self , n : &$ T) -> i8 { match n { 0 => { if self == & 1 { 1 } else { 0 } } 1 => 1 , 2 => { if self % 2 == 0 { 0 } else if self % 8 == 1 || self % 8 == 7 { 1 } else { - 1 } } _ => { let f = n . trailing_zeros () ; let n = n >> f ; self . kronecker (& 2) . pow (f) * self . jacobi (& n) } } } }) *) ; }
};
}
