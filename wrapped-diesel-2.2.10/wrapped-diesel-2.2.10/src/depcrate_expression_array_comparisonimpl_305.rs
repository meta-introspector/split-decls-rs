// Generated macro for impl_305 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_305 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_305"}
// Dependencies: {}
impl < ST , I , QS > SelectableExpression < QS > for Many < ST , I > where Many < ST , I > : AppearsOnTable < QS > , ST : SingleValue , I : AsExpression < ST > , < I as AsExpression < ST > > :: Expression : SelectableExpression < QS > , { }
};
}
