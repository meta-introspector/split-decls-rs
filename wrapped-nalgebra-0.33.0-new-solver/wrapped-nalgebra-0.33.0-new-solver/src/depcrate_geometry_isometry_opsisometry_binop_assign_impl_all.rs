// Generated macro for isometry_binop_assign_impl_all (macro)
macro_rules! Depcrate_geometry_isometry_opsisometry_binop_assign_impl_all {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"isometry_binop_assign_impl_all"}
// Dependencies: {}
macro_rules ! isometry_binop_assign_impl_all (($ OpAssign : ident , $ op_assign : ident ; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty ; [val] => $ action_val : expr ; [ref] => $ action_ref : expr ;) => { impl < T : SimdRealField , R , const D : usize > $ OpAssign <$ Rhs > for $ Lhs where T :: Element : SimdRealField , R : AbstractRotation < T , D > { # [inline] fn $ op_assign (& mut $ lhs , $ rhs : $ Rhs) { $ action_val } } impl <'b , T : SimdRealField , R , const D : usize > $ OpAssign <&'b $ Rhs > for $ Lhs where T :: Element : SimdRealField , R : AbstractRotation < T , D > { # [inline] fn $ op_assign (& mut $ lhs , $ rhs : &'b $ Rhs) { $ action_ref } } }) ;
};
}
