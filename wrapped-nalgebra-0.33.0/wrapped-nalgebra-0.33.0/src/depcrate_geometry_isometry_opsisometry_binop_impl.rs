// Generated macro for isometry_binop_impl (macro)
macro_rules! Depcrate_geometry_isometry_opsisometry_binop_impl {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"isometry_binop_impl"}
// Dependencies: {}
macro_rules ! isometry_binop_impl (($ Op : ident , $ op : ident ; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty , Output = $ Output : ty ; $ action : expr ; $ ($ lives : tt) ,*) => { impl <$ ($ lives ,) * T : SimdRealField , R , const D : usize > $ Op <$ Rhs > for $ Lhs where T :: Element : SimdRealField , R : AbstractRotation < T , D >, { type Output = $ Output ; # [inline] fn $ op ($ lhs , $ rhs : $ Rhs) -> Self :: Output { $ action } } }) ;
};
}
