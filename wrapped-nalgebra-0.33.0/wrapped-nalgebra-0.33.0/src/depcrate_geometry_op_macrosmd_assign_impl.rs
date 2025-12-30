// Generated macro for md_assign_impl (macro)
macro_rules! Depcrate_geometry_op_macrosmd_assign_impl {
() => {
// Module: crate::geometry::op_macros
// Provides: {"md_assign_impl"}
// Dependencies: {}
# [doc = " Macro for the implementation of assignment-multiplication and assignment-division."] macro_rules ! md_assign_impl (($ Op : ident , $ op : ident $ (where T : $ ($ ScalarBounds : ident) ,*) * $ (for T :: Element : $ ($ ElementBounds : ident) ,*) *; ($ R1 : ty , $ C1 : ty) , ($ R2 : ty , $ C2 : ty) const $ ($ D : ident) ,*; for $ ($ DimsDecl : ident) ,*; where $ ($ ConstraintType : ty : $ ConstraintBound : ident $ (<$ ($ ConstraintBoundParams : ty $ (= $ EqBound : ty) *) ,*>) *) ,*; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty ; $ action : expr ; $ ($ lives : tt) ,*) => { impl <$ ($ lives ,) * T $ (, $ DimsDecl) * $ (, const $ D : usize) *> $ Op <$ Rhs > for $ Lhs where T : Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign $ ($ (+ $ ScalarBounds) *) *, $ ($ (T :: Element : $ ElementBounds ,) *) * $ ($ ConstraintType : $ ConstraintBound $ (<$ ($ ConstraintBoundParams $ (= $ EqBound) *) ,*>) *) ,* { # [inline] fn $ op (& mut $ lhs , $ rhs : $ Rhs) { $ action } } }) ;
};
}
