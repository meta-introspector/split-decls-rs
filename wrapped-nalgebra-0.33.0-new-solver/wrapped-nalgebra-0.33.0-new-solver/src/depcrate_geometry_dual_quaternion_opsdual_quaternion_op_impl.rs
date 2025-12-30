// Generated macro for dual_quaternion_op_impl (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsdual_quaternion_op_impl {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"dual_quaternion_op_impl"}
// Dependencies: {}
macro_rules ! dual_quaternion_op_impl (($ OpAssign : ident , $ op_assign : ident ; ($ LhsRDim : ident , $ LhsCDim : ident) , ($ RhsRDim : ident , $ RhsCDim : ident) ; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty $ (=> $ VDimA : ty , $ VDimB : ty) *; $ action : expr ; $ ($ lives : tt) ,*) => { impl <$ ($ lives ,) * T : SimdRealField > $ OpAssign <$ Rhs > for $ Lhs where T :: Element : SimdRealField { # [inline] fn $ op_assign (& mut $ lhs , $ rhs : $ Rhs) { $ action } } }) ;
};
}
