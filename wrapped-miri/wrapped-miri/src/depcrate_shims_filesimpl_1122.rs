// Generated macro for impl_1122 (impl)
macro_rules! Depcrate_shims_filesimpl_1122 {
() => {
// Module: crate::shims::files
// Provides: {"impl_1122"}
// Dependencies: {}
impl FileDescriptionRef < dyn FileDescription > { pub fn downcast < T : FileDescription + 'static > (self) -> Option < FileDescriptionRef < T > > { let inner = self . into_rc_any () . downcast :: < FdIdWith < T > > () . ok () ? ; Some (FileDescriptionRef (inner)) } }
};
}
