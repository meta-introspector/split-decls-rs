// Generated macro for impl_879 (impl)
macro_rules! Depcrate_raw_neoimpl_879 {
() => {
// Module: crate::raw::neo
// Provides: {"impl_879"}
// Dependencies: {}
impl < 'a > TimePatternDataBorrowed < 'a > { pub (crate) fn items_and_options (self) -> ItemsAndOptions < 'a > { let Self :: Resolved (pattern , alignment , hour_cycle , subsecond_digits) = self ; ItemsAndOptions { items : pattern . items , alignment , hour_cycle , subsecond_digits , } } }
};
}
