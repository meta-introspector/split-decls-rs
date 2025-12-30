// Generated macro for impl_2780 (impl)
macro_rules! Depcrate_pg_expression_operatorsimpl_2780 {
() => {
// Module: crate::pg::expression::operators
// Provides: {"impl_2780"}
// Dependencies: {}
impl < L , R > AssignmentTarget for ArrayIndex < L , R > where L : Column , { type Table = < L as Column > :: Table ; type QueryAstNode = ArrayIndex < UncorrelatedColumn < L > , R > ; fn into_target (self) -> Self :: QueryAstNode { ArrayIndex :: new (UncorrelatedColumn (self . array_expr) , self . index_expr) } }
};
}
