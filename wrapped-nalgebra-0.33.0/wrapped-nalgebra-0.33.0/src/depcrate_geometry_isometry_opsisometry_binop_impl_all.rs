// Generated macro for isometry_binop_impl_all (macro)
macro_rules! Depcrate_geometry_isometry_opsisometry_binop_impl_all {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"isometry_binop_impl_all"}
// Dependencies: {}
macro_rules ! isometry_binop_impl_all (($ Op : ident , $ op : ident ; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty , Output = $ Output : ty ; [val val] => $ action_val_val : expr ; [ref val] => $ action_ref_val : expr ; [val ref] => $ action_val_ref : expr ; [ref ref] => $ action_ref_ref : expr ;) => { isometry_binop_impl ! ($ Op , $ op ; $ lhs : $ Lhs , $ rhs : $ Rhs , Output = $ Output ; $ action_val_val ;) ; isometry_binop_impl ! ($ Op , $ op ; $ lhs : &'a $ Lhs , $ rhs : $ Rhs , Output = $ Output ; $ action_ref_val ; 'a) ; isometry_binop_impl ! ($ Op , $ op ; $ lhs : $ Lhs , $ rhs : &'b $ Rhs , Output = $ Output ; $ action_val_ref ; 'b) ; isometry_binop_impl ! ($ Op , $ op ; $ lhs : &'a $ Lhs , $ rhs : &'b $ Rhs , Output = $ Output ; $ action_ref_ref ; 'a , 'b) ; }) ;
};
}
