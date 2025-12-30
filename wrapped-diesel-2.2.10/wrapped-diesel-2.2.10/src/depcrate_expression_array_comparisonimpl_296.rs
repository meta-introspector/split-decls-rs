// Generated macro for impl_296 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_296 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_296"}
// Dependencies: {}
impl < I , T , ST > AsInExpression < ST > for I where I : IntoIterator < Item = T > , T : AsExpression < ST > , ST : SqlType + TypedExpressionType , { type InExpression = Many < ST , T > ; fn as_in_expression (self) -> Self :: InExpression { Many { values : self . into_iter () . collect () , p : PhantomData , } } }
};
}
