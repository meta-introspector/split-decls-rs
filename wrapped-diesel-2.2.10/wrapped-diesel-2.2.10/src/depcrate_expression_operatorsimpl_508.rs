// Generated macro for impl_508 (impl)
macro_rules! Depcrate_expression_operatorsimpl_508 {
() => {
// Module: crate::expression::operators
// Provides: {"impl_508"}
// Dependencies: {}
impl < L , R , ST > crate :: expression :: Expression for Concat < L , R > where L : crate :: expression :: Expression < SqlType = ST > , R : crate :: expression :: Expression < SqlType = ST > , ST : SqlType + TypedExpressionType , { type SqlType = ST ; }
};
}
