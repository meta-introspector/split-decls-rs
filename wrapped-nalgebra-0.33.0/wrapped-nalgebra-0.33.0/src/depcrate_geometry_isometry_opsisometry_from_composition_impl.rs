// Generated macro for isometry_from_composition_impl (macro)
macro_rules! Depcrate_geometry_isometry_opsisometry_from_composition_impl {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"isometry_from_composition_impl"}
// Dependencies: {}
macro_rules ! isometry_from_composition_impl (($ Op : ident , $ op : ident ; $ ($ Dims : ident) ,*; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty , Output = $ Output : ty ; $ action : expr ; $ ($ lives : tt) ,*) => { impl <$ ($ lives ,) * T : SimdRealField $ (, const $ Dims : usize) *> $ Op <$ Rhs > for $ Lhs where T :: Element : SimdRealField { type Output = $ Output ; # [inline] fn $ op ($ lhs , $ rhs : $ Rhs) -> Self :: Output { $ action } } }) ;
};
}
