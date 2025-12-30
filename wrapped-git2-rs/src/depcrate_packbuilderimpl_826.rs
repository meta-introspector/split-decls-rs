// Generated macro for impl_826 (impl)
macro_rules! Depcrate_packbuilderimpl_826 {
() => {
// Module: crate::packbuilder
// Provides: {"impl_826"}
// Dependencies: {}
impl < 'repo > Drop for PackBuilder < 'repo > { fn drop (& mut self) { unsafe { raw :: git_packbuilder_set_callbacks (self . raw , None , ptr :: null_mut ()) ; raw :: git_packbuilder_free (self . raw) ; } } }
};
}
