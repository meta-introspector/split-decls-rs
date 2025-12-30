// Generated macro for impl_1214 (impl)
macro_rules! Depcrate_tagimpl_1214 {
() => {
// Module: crate::tag
// Provides: {"impl_1214"}
// Dependencies: {}
impl < 'repo > Drop for Tag < 'repo > { fn drop (& mut self) { unsafe { raw :: git_tag_free (self . raw) } } }
};
}
