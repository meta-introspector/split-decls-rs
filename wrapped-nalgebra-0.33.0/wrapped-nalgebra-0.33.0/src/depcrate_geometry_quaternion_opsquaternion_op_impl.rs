// Generated macro for quaternion_op_impl (macro)
macro_rules! Depcrate_geometry_quaternion_opsquaternion_op_impl {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"quaternion_op_impl"}
// Dependencies: {}
macro_rules ! quaternion_op_impl (($ OpAssign : ident , $ op_assign : ident ; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty $ (=> $ VDimA : ty , $ VDimB : ty) *; $ action : expr ; $ ($ lives : tt) ,*) => { impl <$ ($ lives ,) * T : SimdRealField > $ OpAssign <$ Rhs > for $ Lhs where T :: Element : SimdRealField { # [inline] fn $ op_assign (& mut $ lhs , $ rhs : $ Rhs) { $ action } } }) ;
};
}
