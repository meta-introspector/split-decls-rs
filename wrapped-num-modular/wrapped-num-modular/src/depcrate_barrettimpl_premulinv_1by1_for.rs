// Generated macro for impl_premulinv_1by1_for (macro)
macro_rules! Depcrate_barrettimpl_premulinv_1by1_for {
() => {
// Module: crate::barrett
// Provides: {"impl_premulinv_1by1_for"}
// Dependencies: {}
macro_rules ! impl_premulinv_1by1_for { ($ T : ty) => { impl PreMulInv1by1 <$ T > { pub const fn new (divisor : $ T) -> Self { debug_assert ! (divisor > 1) ; let n = <$ T >:: BITS - (divisor - 1) . leading_zeros () ; let (lo , _hi) = split (merge (0 , ones (n) - (divisor - 1)) / extend (divisor)) ; debug_assert ! (_hi == 0) ; Self { shift : n - 1 , m : lo + 1 , } } # [doc = " (a / divisor, a % divisor)"] # [inline] pub const fn div_rem (& self , a : $ T , d : $ T) -> ($ T , $ T) { let (_ , t) = split (wmul (self . m , a)) ; let q = (t + ((a - t) >> 1)) >> self . shift ; let r = a - q * d ; (q , r) } } impl DivExact <$ T , PreMulInv1by1 <$ T >> for $ T { type Output = $ T ; # [inline] fn div_exact (self , d : $ T , pre : & PreMulInv1by1 <$ T >) -> Option < Self :: Output > { let (q , r) = pre . div_rem (self , d) ; if r == 0 { Some (q) } else { None } } } } ; }
};
}
