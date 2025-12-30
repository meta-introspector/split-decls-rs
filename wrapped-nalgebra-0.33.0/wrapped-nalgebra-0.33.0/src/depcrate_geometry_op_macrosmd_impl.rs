// Generated macro for md_impl (macro)
macro_rules! Depcrate_geometry_op_macrosmd_impl {
() => {
// Module: crate::geometry::op_macros
// Provides: {"md_impl"}
// Dependencies: {}
# [doc = " Macro for the implementation of multiplication and division."] macro_rules ! md_impl (($ Op : ident , $ op : ident $ (where T : $ ($ ScalarBounds : ident) ,*) *; ($ R1 : ty , $ C1 : ty) , ($ R2 : ty , $ C2 : ty) const $ ($ D : ident) ,*; for $ ($ DimsDecl : ident) ,*; where $ ($ ConstraintType : ty : $ ConstraintBound : ident $ (<$ ($ ConstraintBoundParams : ty $ (= $ EqBound : ty) *) ,*>) *) ,*; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty , Output = $ Result : ty ; $ action : expr ; $ ($ lives : tt) ,*) => { impl <$ ($ lives ,) * T $ (, $ DimsDecl) * $ (, const $ D : usize) *> $ Op <$ Rhs > for $ Lhs where T : Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign $ ($ (+ $ ScalarBounds) *) *, $ ($ ConstraintType : $ ConstraintBound $ (<$ ($ ConstraintBoundParams $ (= $ EqBound) *) ,*>) *) ,* { type Output = $ Result ; # [inline] fn $ op ($ lhs , $ rhs : $ Rhs) -> Self :: Output { $ action } } }) ;
};
}
