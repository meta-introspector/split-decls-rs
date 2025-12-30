// Generated macro for impl_4074 (impl)
macro_rules! Depcrate_type_impls_optionimpl_4074 {
() => {
// Module: crate::type_impls::option
// Provides: {"impl_4074"}
// Dependencies: {}
impl < T , ST > AsExpression < Nullable < ST > > for & Option < T > where ST : SqlType < IsNull = is_nullable :: NotNull > , Nullable < ST > : TypedExpressionType , { type Expression = Bound < Nullable < ST > , Self > ; fn as_expression (self) -> Self :: Expression { Bound :: new (self) } }
};
}
