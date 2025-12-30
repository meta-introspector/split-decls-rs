// Generated macro for impl_reduced_binary_pow (macro)
macro_rules! Depcrate_reducedimpl_reduced_binary_pow {
() => {
// Module: crate::reduced
// Provides: {"impl_reduced_binary_pow"}
// Dependencies: {}
macro_rules ! impl_reduced_binary_pow { ($ T : ty) => { fn pow (& self , base : $ T , exp : &$ T) -> $ T { match * exp { 1 => base , 2 => self . sqr (base) , e => { let mut multi = base ; let mut exp = e ; let mut result = self . transform (1) ; while exp > 0 { if exp & 1 != 0 { result = self . mul (& result , & multi) ; } multi = self . sqr (multi) ; exp >>= 1 ; } result } } } } ; }
};
}
