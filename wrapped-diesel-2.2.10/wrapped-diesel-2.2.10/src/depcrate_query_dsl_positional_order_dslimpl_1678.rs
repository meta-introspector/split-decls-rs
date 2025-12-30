// Generated macro for impl_1678 (impl)
macro_rules! Depcrate_query_dsl_positional_order_dslimpl_1678 {
() => {
// Module: crate::query_dsl::positional_order_dsl
// Provides: {"impl_1678"}
// Dependencies: {}
impl < T : Into < OrderColumn > + Copy > Order for T { type Fragment = OrderColumn ; fn into_fragment (self) -> Self :: Fragment { self . into () } }
};
}
