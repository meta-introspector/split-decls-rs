// Generated macro for impl_746 (impl)
macro_rules! Depcrate_objectimpl_746 {
() => {
// Module: crate::object
// Provides: {"impl_746"}
// Dependencies: {}
impl < 'repo > Drop for Object < 'repo > { fn drop (& mut self) { unsafe { raw :: git_object_free (self . raw) } } }
};
}
