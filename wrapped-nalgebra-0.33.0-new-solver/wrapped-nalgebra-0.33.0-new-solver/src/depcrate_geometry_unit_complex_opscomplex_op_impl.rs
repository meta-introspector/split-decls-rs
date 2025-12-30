// Generated macro for complex_op_impl (macro)
macro_rules! Depcrate_geometry_unit_complex_opscomplex_op_impl {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"complex_op_impl"}
// Dependencies: {}
macro_rules ! complex_op_impl (($ Op : ident , $ op : ident ; $ ($ Storage : ident : $ StoragesBound : ident $ (<$ ($ BoundParam : ty) ,*>) *) ,*; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty , Output = $ Result : ty ; $ action : expr ; $ ($ lives : tt) ,*) => { impl <$ ($ lives ,) * T : SimdRealField $ (, $ Storage : $ StoragesBound $ (<$ ($ BoundParam) ,*>) *) *> $ Op <$ Rhs > for $ Lhs where T :: Element : SimdRealField { type Output = $ Result ; # [inline] fn $ op ($ lhs , $ rhs : $ Rhs) -> Self :: Output { $ action } } }) ;
};
}
