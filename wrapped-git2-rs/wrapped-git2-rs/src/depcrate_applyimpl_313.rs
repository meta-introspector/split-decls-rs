// Generated macro for impl_313 (impl)
macro_rules! Depcrate_applyimpl_313 {
() => {
// Module: crate::apply
// Provides: {"impl_313"}
// Dependencies: {}
impl Binding for ApplyLocation { type Raw = raw :: git_apply_location_t ; unsafe fn from_raw (raw : raw :: git_apply_location_t) -> Self { match raw { raw :: GIT_APPLY_LOCATION_WORKDIR => Self :: WorkDir , raw :: GIT_APPLY_LOCATION_INDEX => Self :: Index , raw :: GIT_APPLY_LOCATION_BOTH => Self :: Both , _ => panic ! ("Unknown git diff binary kind") , } } fn raw (& self) -> raw :: git_apply_location_t { match * self { Self :: WorkDir => raw :: GIT_APPLY_LOCATION_WORKDIR , Self :: Index => raw :: GIT_APPLY_LOCATION_INDEX , Self :: Both => raw :: GIT_APPLY_LOCATION_BOTH , } } }
};
}
