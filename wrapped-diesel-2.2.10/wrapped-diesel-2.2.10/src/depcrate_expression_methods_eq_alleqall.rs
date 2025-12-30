// Generated macro for EqAll (trait)
macro_rules! Depcrate_expression_methods_eq_allEqAll {
() => {
// Module: crate::expression_methods::eq_all
// Provides: {"EqAll"}
// Dependencies: {}
# [doc = " This method is used by `FindDsl` to work with tuples. Because we cannot"] # [doc = " express this without specialization or overlapping impls, it is brute force"] # [doc = " implemented on columns in the `column!` macro."] # [doc (hidden)] pub trait EqAll < Rhs > { type Output : Expression < SqlType = Bool > ; fn eq_all (self , rhs : Rhs) -> Self :: Output ; }
};
}
