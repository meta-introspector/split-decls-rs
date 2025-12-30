// Generated macro for impl_635 (impl)
macro_rules! Depcrate_displayimpl_635 {
() => {
// Module: crate::display
// Provides: {"impl_635"}
// Dependencies: {}
impl < 'a , T > ExpressionStoreAdapter < 'a , T > { fn wrap (store : & 'a ExpressionStore) -> impl Fn (T) -> ExpressionStoreAdapter < 'a , T > { move | value | ExpressionStoreAdapter (value , store) } }
};
}
