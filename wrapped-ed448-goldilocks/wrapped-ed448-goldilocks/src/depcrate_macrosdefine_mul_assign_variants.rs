// Generated macro for define_mul_assign_variants (macro)
macro_rules! Depcrate_macrosdefine_mul_assign_variants {
() => {
// Module: crate::macros
// Provides: {"define_mul_assign_variants"}
// Dependencies: {}
# [doc = " Define non-borrow variants of `MulAssign`."] macro_rules ! define_mul_assign_variants { (LHS = $ lhs : ty , RHS = $ rhs : ty) => { impl MulAssign <$ rhs > for $ lhs { fn mul_assign (& mut self , rhs : $ rhs) { * self *= & rhs ; } } } ; }
};
}
