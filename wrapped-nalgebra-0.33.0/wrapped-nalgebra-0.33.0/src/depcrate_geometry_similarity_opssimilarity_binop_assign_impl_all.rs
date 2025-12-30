// Generated macro for similarity_binop_assign_impl_all (macro)
macro_rules! Depcrate_geometry_similarity_opssimilarity_binop_assign_impl_all {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"similarity_binop_assign_impl_all"}
// Dependencies: {}
macro_rules ! similarity_binop_assign_impl_all (($ OpAssign : ident , $ op_assign : ident ; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty ; [val] => $ action_val : expr ; [ref] => $ action_ref : expr ;) => { impl < T : SimdRealField , R , const D : usize > $ OpAssign <$ Rhs > for $ Lhs where T :: Element : SimdRealField , R : AbstractRotation < T , D > { # [inline] fn $ op_assign (& mut $ lhs , $ rhs : $ Rhs) { $ action_val } } impl <'b , T : SimdRealField , R , const D : usize > $ OpAssign <&'b $ Rhs > for $ Lhs where T :: Element : SimdRealField , R : AbstractRotation < T , D > { # [inline] fn $ op_assign (& mut $ lhs , $ rhs : &'b $ Rhs) { $ action_ref } } }) ;
};
}
