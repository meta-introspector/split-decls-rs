// Generated macro for impl_440 (impl)
macro_rules! Depcrate_meta_strategyimpl_440 {
() => {
// Module: crate::meta::strategy
// Provides: {"impl_440"}
// Dependencies: {}
impl < P : PrefilterI > Pre < P > { fn new (pre : P) -> Arc < dyn Strategy > { let group_info = GroupInfo :: new ([[None :: < & str >]]) . unwrap () ; Arc :: new (Pre { pre , group_info }) } }
};
}
