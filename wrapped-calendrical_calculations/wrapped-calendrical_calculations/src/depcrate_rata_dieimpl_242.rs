// Generated macro for impl_242 (impl)
macro_rules! Depcrate_rata_dieimpl_242 {
() => {
// Module: crate::rata_die
// Provides: {"impl_242"}
// Dependencies: {}
impl fmt :: Debug for RataDie { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let rd = self . 0 ; if let Ok ((y , m , d)) = crate :: gregorian :: gregorian_from_fixed (* self) { write ! (f , "{rd} R.D. ({y}-{m:02}-{d:02})") } else { write ! (f , "{rd} R.D. (out of bounds)") } } }
};
}
