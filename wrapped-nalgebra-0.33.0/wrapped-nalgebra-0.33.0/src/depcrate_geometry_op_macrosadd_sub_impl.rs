// Generated macro for add_sub_impl (macro)
macro_rules! Depcrate_geometry_op_macrosadd_sub_impl {
() => {
// Module: crate::geometry::op_macros
// Provides: {"add_sub_impl"}
// Dependencies: {}
# [doc = " Macro for the implementation of addition and subtraction."] macro_rules ! add_sub_impl (($ Op : ident , $ op : ident , $ bound : ident ; ($ R1 : ty , $ C1 : ty) , ($ R2 : ty , $ C2 : ty) $ (-> ($ RRes : ty , $ CRes : ty)) * const $ ($ D : ident) ,*; for $ ($ DimsDecl : ident) ,*; where $ ($ ConstraintType : ty : $ ConstraintBound : ident $ (<$ ($ ConstraintBoundParams : ty $ (= $ EqBound : ty) *) ,*>) *) ,*; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty , Output = $ Result : ty ; $ action : expr ; $ ($ lives : tt) ,*) => { impl <$ ($ lives ,) * T $ (, $ DimsDecl) * $ (, const $ D : usize) *> $ Op <$ Rhs > for $ Lhs where T : Scalar + $ bound , ShapeConstraint : SameNumberOfRows <$ R1 , $ R2 $ (, Representative = $ RRes) *> + SameNumberOfColumns <$ C1 , $ C2 $ (, Representative = $ CRes) *>, $ ($ ConstraintType : $ ConstraintBound $ (<$ ($ ConstraintBoundParams $ (= $ EqBound) *) ,*>) *) ,* { type Output = $ Result ; # [inline] fn $ op ($ lhs , $ rhs : $ Rhs) -> Self :: Output { $ action } } }) ;
};
}
