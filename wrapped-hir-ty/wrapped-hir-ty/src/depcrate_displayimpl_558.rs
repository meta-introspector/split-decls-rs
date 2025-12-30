// Generated macro for impl_558 (impl)
macro_rules! Depcrate_displayimpl_558 {
() => {
// Module: crate::display
// Provides: {"impl_558"}
// Dependencies: {}
impl < 'a , T > ExpressionStoreAdapter < 'a , T > { fn wrap (store : & 'a ExpressionStore) -> impl Fn (T) -> ExpressionStoreAdapter < 'a , T > { move | value | ExpressionStoreAdapter (value , store) } }
};
}
