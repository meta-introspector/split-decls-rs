// Generated macro for impl_2777 (impl)
macro_rules! Depcrate_pg_expression_operatorsimpl_2777 {
() => {
// Module: crate::pg::expression::operators
// Provides: {"impl_2777"}
// Dependencies: {}
impl < L , R , ST > crate :: expression :: Expression for ArrayIndex < L , R > where L : crate :: expression :: Expression < SqlType = Array < ST > > , R : crate :: expression :: Expression < SqlType = Integer > , ST : SqlType + TypedExpressionType , { type SqlType = ST ; }
};
}
