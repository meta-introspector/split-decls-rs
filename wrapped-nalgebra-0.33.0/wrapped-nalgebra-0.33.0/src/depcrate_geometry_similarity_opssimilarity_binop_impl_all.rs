// Generated macro for similarity_binop_impl_all (macro)
macro_rules! Depcrate_geometry_similarity_opssimilarity_binop_impl_all {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"similarity_binop_impl_all"}
// Dependencies: {}
macro_rules ! similarity_binop_impl_all (($ Op : ident , $ op : ident ; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty , Output = $ Output : ty ; [val val] => $ action_val_val : expr ; [ref val] => $ action_ref_val : expr ; [val ref] => $ action_val_ref : expr ; [ref ref] => $ action_ref_ref : expr ;) => { similarity_binop_impl ! ($ Op , $ op ; $ lhs : $ Lhs , $ rhs : $ Rhs , Output = $ Output ; $ action_val_val ;) ; similarity_binop_impl ! ($ Op , $ op ; $ lhs : &'a $ Lhs , $ rhs : $ Rhs , Output = $ Output ; $ action_ref_val ; 'a) ; similarity_binop_impl ! ($ Op , $ op ; $ lhs : $ Lhs , $ rhs : &'b $ Rhs , Output = $ Output ; $ action_val_ref ; 'b) ; similarity_binop_impl ! ($ Op , $ op ; $ lhs : &'a $ Lhs , $ rhs : &'b $ Rhs , Output = $ Output ; $ action_ref_ref ; 'a , 'b) ; }) ;
};
}
