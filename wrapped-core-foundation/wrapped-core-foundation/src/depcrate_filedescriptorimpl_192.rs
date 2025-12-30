// Generated macro for impl_192 (impl)
macro_rules! Depcrate_filedescriptorimpl_192 {
() => {
// Module: crate::filedescriptor
// Provides: {"impl_192"}
// Dependencies: {}
impl AsRawFd for CFFileDescriptor { fn as_raw_fd (& self) -> RawFd { unsafe { CFFileDescriptorGetNativeDescriptor (self . 0) } } }
};
}
