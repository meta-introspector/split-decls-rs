// Generated macro for EqAny (type)
macro_rules! Depcrate_expression_helper_typesEqAny {
() => {
// Module: crate::expression::helper_types
// Provides: {"EqAny"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.eq_any(rhs)`](crate::expression_methods::ExpressionMethods::eq_any())"] pub type EqAny < Lhs , Rhs > = Grouped < In < Lhs , < Rhs as AsInExpression < SqlTypeOf < Lhs > > > :: InExpression > > ;
};
}
