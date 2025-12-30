// Generated macro for impl_302 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_302 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_302"}
// Dependencies: {}
impl < ST , I , GB > ValidGrouping < GB > for Many < ST , I > where ST : SingleValue , I : AsExpression < ST > , I :: Expression : ValidGrouping < GB > , { type IsAggregate = < I :: Expression as ValidGrouping < GB > > :: IsAggregate ; }
};
}
