// Generated macro for impl_1213 (impl)
macro_rules! Depcrate_type_check_liveness_local_use_mapimpl_1213 {
() => {
// Module: crate::type_check::liveness::local_use_map
// Provides: {"impl_1213"}
// Dependencies: {}
impl < 'a > Iterator for AppearancesIter < 'a > { type Item = AppearanceIndex ; fn next (& mut self) -> Option < AppearanceIndex > { if let Some (c) = self . current { self . current = self . appearances [c] . next ; Some (c) } else { None } } }
};
}
