// Generated macro for impl_387 (impl)
macro_rules! Depcrate_expression_existsimpl_387 {
() => {
// Module: crate::expression::exists
// Provides: {"impl_387"}
// Dependencies: {}
impl < T , GB > ValidGrouping < GB > for Exists < T > where Subselect < T , Bool > : ValidGrouping < GB > , { type IsAggregate = < Subselect < T , Bool > as ValidGrouping < GB > > :: IsAggregate ; }
};
}
