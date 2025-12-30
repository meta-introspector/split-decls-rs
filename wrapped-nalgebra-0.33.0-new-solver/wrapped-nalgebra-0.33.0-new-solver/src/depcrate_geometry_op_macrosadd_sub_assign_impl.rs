// Generated macro for add_sub_assign_impl (macro)
macro_rules! Depcrate_geometry_op_macrosadd_sub_assign_impl {
() => {
// Module: crate::geometry::op_macros
// Provides: {"add_sub_assign_impl"}
// Dependencies: {}
# [doc = " Macro for the implementation of assignment-addition and assignment-subtraction."] macro_rules ! add_sub_assign_impl (($ Op : ident , $ op : ident , $ bound : ident ; $ (const $ D : ident) ,*; $ lhs : ident : $ Lhs : ty , $ rhs : ident : $ Rhs : ty ; $ action : expr ; $ ($ lives : tt) ,*) => { impl <$ ($ lives ,) * T $ (, const $ D : usize) ,*> $ Op <$ Rhs > for $ Lhs where T : Scalar + $ bound { # [inline] fn $ op (& mut $ lhs , $ rhs : $ Rhs) { $ action } } }) ;
};
}
