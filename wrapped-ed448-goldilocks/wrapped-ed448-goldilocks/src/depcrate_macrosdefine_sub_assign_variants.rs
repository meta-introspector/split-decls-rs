// Generated macro for define_sub_assign_variants (macro)
macro_rules! Depcrate_macrosdefine_sub_assign_variants {
() => {
// Module: crate::macros
// Provides: {"define_sub_assign_variants"}
// Dependencies: {}
# [doc = " Define non-borrow variants of `SubAssign`."] macro_rules ! define_sub_assign_variants { (LHS = $ lhs : ty , RHS = $ rhs : ty) => { impl SubAssign <$ rhs > for $ lhs { fn sub_assign (& mut self , rhs : $ rhs) { * self -= & rhs ; } } } ; }
};
}
