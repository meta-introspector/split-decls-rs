// Generated macro for define_add_assign_variants (macro)
macro_rules! Depcrate_macrosdefine_add_assign_variants {
() => {
// Module: crate::macros
// Provides: {"define_add_assign_variants"}
// Dependencies: {}
# [doc = " Define non-borrow variants of `AddAssign`."] macro_rules ! define_add_assign_variants { (LHS = $ lhs : ty , RHS = $ rhs : ty) => { impl AddAssign <$ rhs > for $ lhs { fn add_assign (& mut self , rhs : $ rhs) { * self += & rhs ; } } } ; }
};
}
