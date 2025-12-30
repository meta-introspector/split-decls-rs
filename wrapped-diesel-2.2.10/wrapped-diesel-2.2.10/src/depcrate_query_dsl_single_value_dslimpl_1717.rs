// Generated macro for impl_1717 (impl)
macro_rules! Depcrate_query_dsl_single_value_dslimpl_1717 {
() => {
// Module: crate::query_dsl::single_value_dsl
// Provides: {"impl_1717"}
// Dependencies: {}
impl < T > SingleValueDsl for T where Self : SelectQuery + LimitDsl , < Self as SelectQuery > :: SqlType : IntoNullable , { type Output = Grouped < Subselect < Limit < Self > , < < Self as SelectQuery > :: SqlType as IntoNullable > :: Nullable > > ; fn single_value (self) -> Self :: Output { Grouped (Subselect :: new (self . limit (1))) } }
};
}
