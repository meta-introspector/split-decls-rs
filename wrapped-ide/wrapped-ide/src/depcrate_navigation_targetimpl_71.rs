// Generated macro for impl_71 (impl)
macro_rules! Depcrate_navigation_targetimpl_71 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_71"}
// Dependencies: {}
impl < T > UpmappingResult < T > { pub (crate) fn map < U > (self , f : impl Fn (T) -> U) -> UpmappingResult < U > { UpmappingResult { call_site : f (self . call_site) , def_site : self . def_site . map (f) } } }
};
}
