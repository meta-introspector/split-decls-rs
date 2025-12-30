// Generated macro for impl_881 (impl)
macro_rules! Depcrate_raw_neoimpl_881 {
() => {
// Module: crate::raw::neo
// Provides: {"impl_881"}
// Dependencies: {}
impl < 'a > ZonePatternDataBorrowed < 'a > { pub (crate) fn items_and_options (self) -> ItemsAndOptions < 'a > { let Self :: SinglePatternItem (item) = self ; ItemsAndOptions { items : ZeroSlice :: from_ule_slice (core :: slice :: from_ref (item)) , .. Default :: default () } } }
};
}
