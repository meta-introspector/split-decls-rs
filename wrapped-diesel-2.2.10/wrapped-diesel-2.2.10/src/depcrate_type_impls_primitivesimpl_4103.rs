// Generated macro for impl_4103 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4103 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4103"}
// Dependencies: {}
impl < 'a , T : ? Sized , ST > AsExpression < ST > for Cow < 'a , T > where T : 'a + ToOwned , Bound < ST , Cow < 'a , T > > : Expression < SqlType = ST > , ST : SqlType + TypedExpressionType , { type Expression = Bound < ST , Self > ; fn as_expression (self) -> Self :: Expression { Bound :: new (self) } }
};
}
