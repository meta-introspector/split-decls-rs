// Generated macro for impl_4104 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4104 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4104"}
// Dependencies: {}
impl < 'a , 'b , T : ? Sized , ST > AsExpression < ST > for & 'b Cow < 'a , T > where T : 'a + ToOwned , Bound < ST , & 'b T > : Expression < SqlType = ST > , ST : SqlType + TypedExpressionType , { type Expression = Bound < ST , & 'b T > ; fn as_expression (self) -> Self :: Expression { Bound :: new (& * * self) } }
};
}
