// Generated macro for impl_4077 (impl)
macro_rules! Depcrate_type_impls_optionimpl_4077 {
() => {
// Module: crate::type_impls::option
// Provides: {"impl_4077"}
// Dependencies: {}
impl < T , DB > Selectable < DB > for Option < T > where DB : Backend , T : Selectable < DB > , crate :: dsl :: Nullable < T :: SelectExpression > : Expression , { type SelectExpression = crate :: dsl :: Nullable < T :: SelectExpression > ; fn construct_selection () -> Self :: SelectExpression { T :: construct_selection () . nullable () } }
};
}
