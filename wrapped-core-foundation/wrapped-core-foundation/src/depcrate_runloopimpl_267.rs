// Generated macro for impl_267 (impl)
macro_rules! Depcrate_runloopimpl_267 {
() => {
// Module: crate::runloop
// Provides: {"impl_267"}
// Dependencies: {}
impl CFRunLoopSource { pub fn from_file_descriptor (fd : & CFFileDescriptor , order : CFIndex) -> Option < CFRunLoopSource > { fd . to_run_loop_source (order) } }
};
}
