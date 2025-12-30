// Generated macro for impl_68 (impl)
macro_rules! Depcrate_navigation_targetimpl_68 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_68"}
// Dependencies: {}
impl < T > IntoIterator for UpmappingResult < T > { type Item = T ; type IntoIter = < ArrayVec < T , 2 > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . def_site . into_iter () . chain (Some (self . call_site)) . collect :: < ArrayVec < _ , 2 > > () . into_iter () } }
};
}
