// Generated macro for impl_div_exact_for_prim (macro)
macro_rules! Depcrate_primimpl_div_exact_for_prim {
() => {
// Module: crate::prim
// Provides: {"impl_div_exact_for_prim"}
// Dependencies: {}
macro_rules ! impl_div_exact_for_prim { ($ ($ t : ty) *) => { $ (impl DivExact <$ t , () > for $ t { type Output = $ t ; # [inline] fn div_exact (self , d : $ t , _ : & ()) -> Option < Self :: Output > { let (q , r) = (self / d , self % d) ; if r == 0 { Some (q) } else { None } } }) * } ; }
};
}
