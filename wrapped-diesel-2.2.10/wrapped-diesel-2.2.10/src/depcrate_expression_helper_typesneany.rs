// Generated macro for NeAny (type)
macro_rules! Depcrate_expression_helper_typesNeAny {
() => {
// Module: crate::expression::helper_types
// Provides: {"NeAny"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.ne_all(rhs)`](crate::expression_methods::ExpressionMethods::ne_all())"] pub type NeAny < Lhs , Rhs > = Grouped < NotIn < Lhs , < Rhs as AsInExpression < SqlTypeOf < Lhs > > > :: InExpression > > ;
};
}
