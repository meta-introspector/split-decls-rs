// Generated macro for md_assign_impl_all (macro)
macro_rules! Depcrate_geometry_op_macrosmd_assign_impl_all {
() => {
// Module: crate::geometry::op_macros
// Provides: {"md_assign_impl_all"}
// Dependencies: {}
# [doc = " Macro for the implementation of assignment-multiplication and assignment-division with and"] # [doc = " without reference to the right-hand-side."] macro_rules ! md_assign_impl_all (($ Op : ident , $ op : ident $ (where T : $ ($ ScalarBounds : ident) ,*) * $ (for T :: Element : $ ($ ElementBounds : ident) ,*) *; ($ R1 : ty , $ C1 : ty) , ($ R2 : ty , $ C2 : ty) const $ ($ D : ident) ,*; for $ ($ DimsDecl : ident) ,*; where $ ($ ConstraintType : ty : $ ConstraintBound : ident $ (<$ ($ ConstraintBoundParams : ty $ (= $ EqBound : ty) *) ,*>) *) ,*; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty ; [val] => $ action_val : expr ; [ref] => $ action_ref : expr ;) => { md_assign_impl ! ($ Op , $ op $ (where T : $ ($ ScalarBounds) ,*) * $ (for T :: Element : $ ($ ElementBounds) ,*) *; ($ R1 , $ C1) , ($ R2 , $ C2) const $ ($ D) ,*; for $ ($ DimsDecl) ,*; where $ ($ ConstraintType : $ ConstraintBound $ (<$ ($ ConstraintBoundParams $ (= $ EqBound) *) ,*>) *) ,*; $ lhs : $ Lhs , $ rhs : $ Rhs ; $ action_val ;) ; md_assign_impl ! ($ Op , $ op $ (where T : $ ($ ScalarBounds) ,*) * $ (for T :: Element : $ ($ ElementBounds) ,*) *; ($ R1 , $ C1) , ($ R2 , $ C2) const $ ($ D) ,*; for $ ($ DimsDecl) ,*; where $ ($ ConstraintType : $ ConstraintBound $ (<$ ($ ConstraintBoundParams $ (= $ EqBound) *) ,*>) *) ,*; $ lhs : $ Lhs , $ rhs : &'b $ Rhs ; $ action_ref ; 'b) ; }) ;
};
}
