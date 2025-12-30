// Generated macro for pow_impl (macro)
macro_rules! Depcrate_powpow_impl {
() => {
// Module: crate::pow
// Provides: {"pow_impl"}
// Dependencies: {}
macro_rules ! pow_impl { ($ U : ty , $ S : ty) => { impl <'a , T : Clone + Num > Pow <$ U > for &'a Complex < T > { type Output = Complex < T >; # [inline] fn pow (self , mut exp : $ U) -> Self :: Output { if exp == 0 { return Complex :: one () ; } let mut base = self . clone () ; while exp & 1 == 0 { base = base . clone () * base ; exp >>= 1 ; } if exp == 1 { return base ; } let mut acc = base . clone () ; while exp > 1 { exp >>= 1 ; base = base . clone () * base ; if exp & 1 == 1 { acc = acc * base . clone () ; } } acc } } impl <'a , 'b , T : Clone + Num > Pow <&'b $ U > for &'a Complex < T > { type Output = Complex < T >; # [inline] fn pow (self , exp : &$ U) -> Self :: Output { self . pow (* exp) } } impl <'a , T : Clone + Num + Neg < Output = T >> Pow <$ S > for &'a Complex < T > { type Output = Complex < T >; # [inline] fn pow (self , exp : $ S) -> Self :: Output { if exp < 0 { Pow :: pow (& self . inv () , exp . wrapping_neg () as $ U) } else { Pow :: pow (self , exp as $ U) } } } impl <'a , 'b , T : Clone + Num + Neg < Output = T >> Pow <&'b $ S > for &'a Complex < T > { type Output = Complex < T >; # [inline] fn pow (self , exp : &$ S) -> Self :: Output { self . pow (* exp) } } } ; }
};
}
