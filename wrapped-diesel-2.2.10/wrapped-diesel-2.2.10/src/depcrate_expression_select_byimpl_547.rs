// Generated macro for impl_547 (impl)
macro_rules! Depcrate_expression_select_byimpl_547 {
() => {
// Module: crate::expression::select_by
// Provides: {"impl_547"}
// Dependencies: {}
impl < T , GB , E , DB > ValidGrouping < GB > for SelectBy < T , DB > where DB : Backend , T : Selectable < DB , SelectExpression = E > , E : Expression + ValidGrouping < GB > , { type IsAggregate = E :: IsAggregate ; }
};
}
