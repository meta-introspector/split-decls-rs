// Generated macro for impl_860 (impl)
macro_rules! Depcrate_config_tree_sections_checkoutimpl_860 {
() => {
// Module: crate::config::tree::sections::checkout
// Provides: {"impl_860"}
// Dependencies: {}
impl Checkout { # [doc = " The `checkout.workers` key."] pub const WORKERS : Workers = Workers :: new_with_validate ("workers" , & config :: Tree :: CHECKOUT , validate :: Workers) . with_deviation ("if unset, uses all cores instead of just one") ; }
};
}
