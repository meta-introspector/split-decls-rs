// Generated macro for complex_op_impl_all (macro)
macro_rules! Depcrate_geometry_unit_complex_opscomplex_op_impl_all {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"complex_op_impl_all"}
// Dependencies: {}
macro_rules ! complex_op_impl_all (($ Op : ident , $ op : ident ; $ ($ Storage : ident : $ StoragesBound : ident $ (<$ ($ BoundParam : ty) ,*>) *) ,*; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty , Output = $ Result : ty ; [val val] => $ action_val_val : expr ; [ref val] => $ action_ref_val : expr ; [val ref] => $ action_val_ref : expr ; [ref ref] => $ action_ref_ref : expr ;) => { complex_op_impl ! ($ Op , $ op ; $ ($ Storage : $ StoragesBound $ (<$ ($ BoundParam) ,*>) *) ,*; $ lhs : $ Lhs , $ rhs : $ Rhs , Output = $ Result ; $ action_val_val ;) ; complex_op_impl ! ($ Op , $ op ; $ ($ Storage : $ StoragesBound $ (<$ ($ BoundParam) ,*>) *) ,*; $ lhs : &'a $ Lhs , $ rhs : $ Rhs , Output = $ Result ; $ action_ref_val ; 'a) ; complex_op_impl ! ($ Op , $ op ; $ ($ Storage : $ StoragesBound $ (<$ ($ BoundParam) ,*>) *) ,*; $ lhs : $ Lhs , $ rhs : &'b $ Rhs , Output = $ Result ; $ action_val_ref ; 'b) ; complex_op_impl ! ($ Op , $ op ; $ ($ Storage : $ StoragesBound $ (<$ ($ BoundParam) ,*>) *) ,*; $ lhs : &'a $ Lhs , $ rhs : &'b $ Rhs , Output = $ Result ; $ action_ref_ref ; 'a , 'b) ; }) ;
};
}
